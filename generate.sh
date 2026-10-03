#!/bin/bash
set -euo pipefail

# Regenerate every Customer.io API module from the upstream OpenAPI specs.
#
# Customer.io ships one spec per API (Track, App, Pipelines), each on its own host. We generate
# each into a temp dir, then vendor only its `src/apis` and `src/models` into `src/<service>/`,
# rewriting the generator's absolute `crate::apis` / `crate::models` paths to
# `crate::<service>::…` so the code compiles inside a submodule. The run is idempotent: an
# unchanged spec reproduces the committed tree.
#
# Requires only a JDK. The openapi-generator version is pinned and the JAR is fetched directly
# from Maven Central (cached under ~/.cache), so local and CI runs are byte-for-byte identical —
# no dependence on a brew/npm install whose default generator version drifts.

# Portable in-place sed (macOS uses -i '', Linux uses -i).
sedi() {
  if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' "$@"
  else
    sed -i "$@"
  fi
}

SPEC_BASE="https://docs.customer.io/files"
# "<upstream spec file stem>:<rust module name>" pairs. The module name doubles as the Cargo
# feature name.
SERVICES="
journeys-track:track
journeys-app:app
pipelines:pipelines
"

# Pinned generator version. Bumping this is a deliberate act (it can change generated output, e.g.
# the String -> chrono date-time switch in 7.15+); regenerate and review the diff when you change it.
GENERATOR_VERSION="${GENERATOR_VERSION:-7.23.0}"
JAR_CACHE="${XDG_CACHE_HOME:-$HOME/.cache}/openapi-generator"
JAR="$JAR_CACHE/openapi-generator-cli-${GENERATOR_VERSION}.jar"
if [ ! -f "$JAR" ]; then
  echo "==> Fetching openapi-generator ${GENERATOR_VERSION} JAR..."
  mkdir -p "$JAR_CACHE"
  curl -sS -L --fail -o "$JAR" \
    "https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/${GENERATOR_VERSION}/openapi-generator-cli-${GENERATOR_VERSION}.jar"
fi
GEN=(java -jar "$JAR")

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

echo "==> Fetching Customer.io OpenAPI specs from $SPEC_BASE ..."
for pair in $SERVICES; do
  spec="${pair%%:*}"
  curl -sS -L --fail -o "$WORK/$spec.json" "$SPEC_BASE/$spec.json"
  echo "    $spec.json: $(wc -c < "$WORK/$spec.json" | tr -d ' ') bytes"
done

echo "==> Recording combined spec hash..."
( cd "$WORK" && shasum -a 256 $(for pair in $SERVICES; do echo "${pair%%:*}.json"; done) \
  | sort | shasum -a 256 | awk '{print $1}' ) > SPEC_HASH
echo "    SPEC_HASH = $(cat SPEC_HASH)"

echo "==> Patching specs before generation..."
# The App spec leaves every Assets and Design Studio operation without a security requirement,
# so the generator would send them unauthenticated. Default the whole document to bearer auth;
# operations with their own `security` (including explicit `[]`) are unchanged.
python3 scripts/fix-default-security.py "$WORK/journeys-app.json" Bearer-Auth
# The App spec also documents the inbound reporting-webhook payloads. No operation uses them and
# their tags are inconsistent upstream, so they cannot be generated correctly; drop them.
python3 scripts/drop-webhook-schemas.py "$WORK/journeys-app.json"
SPECS=$(for pair in $SERVICES; do echo "$WORK/${pair%%:*}.json"; done)
# The specs are fully dereferenced (no `$ref`), so every shared schema would become several
# duplicate Rust types. Put the references back first; the passes below then see one copy.
python3 scripts/rereference-schemas.py $SPECS
# Titles with punctuation ("SMS / MMS") make the generator name a model file and its `pub mod`
# line differently. Keep only alphanumeric words.
python3 scripts/fix-schema-titles.py $SPECS
# The App spec repeats a whole parameter list on one operation (duplicate Rust arguments).
python3 scripts/fix-duplicate-parameters.py $SPECS
# Some enums repeat a value ("center" twice), which yields an unreachable duplicate variant.
python3 scripts/fix-duplicate-enum-values.py $SPECS
# The Track v2 entity/batch payloads discriminate on `action` without a mapping, so the
# generator would use model names ("Person__Identify") as tag values. Derive the mapping from
# each member's one-value `action` enum.
python3 scripts/fix-discriminator-mapping.py $SPECS
# The specs are OpenAPI 3.1, and the rust generator crashes on untitled inline scalar `oneOf`
# members under 3.1. Rewrite nullable scalars to `type: [T, "null"]` and title scalar unions.
python3 scripts/fix-inline-oneof.py $SPECS
# The generator hard-codes i32 for a format-less integer, but timestamps and IDs need i64.
python3 scripts/fix-integer-format.py $SPECS

for pair in $SERVICES; do
  spec="${pair%%:*}"
  module="${pair##*:}"
  echo "==> Generating '$spec' -> src/$module ..."

  rm -rf "$WORK/gen-$spec"
  "${GEN[@]}" generate \
    -i "$WORK/$spec.json" \
    -g rust \
    --library reqwest \
    --skip-validate-spec \
    --additional-properties=packageName=autogen-customerio-$module,supportAsync=true,supportMiddleware=true \
    -o "$WORK/gen-$spec" \
    2>&1 | tail -3

  rm -rf "src/$module"
  mkdir -p "src/$module"
  cp -R "$WORK/gen-$spec/src/apis" "src/$module/apis"
  cp -R "$WORK/gen-$spec/src/models" "src/$module/models"

  # Rewrite absolute crate paths so the vendored code resolves inside src/$module/.
  # Order matters: the grouped-import form `crate::{apis::…, models}` is rewritten first; the
  # standalone `crate::apis` / `crate::models` forms are then caught without double-rewriting.
  while IFS= read -r -d '' f; do
    sedi \
      -e "s/crate::{/crate::$module::{/g" \
      -e "s/crate::apis/crate::$module::apis/g" \
      -e "s/crate::models/crate::$module::models/g" \
      "$f"
  done < <(find "src/$module" -name '*.rs' -print0)

  printf 'pub mod apis;\npub mod models;\n' > "src/$module/mod.rs"
done

echo "==> Applying post-generation fixes..."
# Guard against specs declaring two properties that differ only in case; the rust generator
# snake_cases both to one field, producing an uncompilable duplicate. This idempotent pass
# renames the earlier collision to <name>_legacy<n>.
python3 scripts/fix-duplicate-fields.py src

# The generator serializes chrono DateTime query params via Display ("2026-07-17 00:00:00 +00:00"),
# which is not RFC 3339. Rewrite those sites to to_rfc3339_opts.
python3 scripts/fix-datetime-query-params.py src

# Promote allowlisted entity IDs (broadcast_id, campaign_id, delivery_id, …) and the `id` of
# entity models to the strict newtypes in src/ids.rs, so transposed ID arguments become compile
# errors. Allowlist-based: unknown IDs stay raw. Keep scripts/fix-id-types.py and src/ids.rs
# in sync. Guarded by tests/ids.rs.
python3 scripts/fix-id-types.py src

# The rust generator emits closed Rust enums, so one value the spec does not list yet fails
# deserialization of the whole response. Add an `#[serde(untagged)] UnknownValue(String)`
# fallback to every generated string enum. Guarded by tests/open_enums.rs.
python3 scripts/fix-open-enums.py src

echo "==> Verifying compilation..."
cargo check --all-features
for api in track app pipelines; do
  cargo check --no-default-features --features "$api,native-tls"
done

echo "==> Done. Review changes with: git diff"
