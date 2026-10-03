# autogen-customerio

[![CI](https://github.com/steventhanna/autogen-customerio/actions/workflows/ci.yml/badge.svg)](https://github.com/steventhanna/autogen-customerio/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/autogen-customerio.svg)](https://crates.io/crates/autogen-customerio)
[![docs.rs](https://img.shields.io/docsrs/autogen-customerio)](https://docs.rs/autogen-customerio)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](#license)

Auto-generated, strongly-typed, async Rust client for the [Customer.io APIs](https://docs.customer.io/api/).

Every request/response type and API method is generated directly from Customer.io's published
OpenAPI specs with [openapi-generator](https://openapi-generator.tech/), so the surface stays
faithful to the APIs and updates automatically when a spec changes. A thin hand-written
`CustomerIoClient` adds region selection and authentication on top.

- **Complete** — the Journeys Track API, the Journeys App API, and the Data Pipelines API, every
  endpoint and model.
- **Typed IDs** — `BroadcastId`, `CampaignId`, `DeliveryId`, … instead of bare integers and
  strings, so a transposed argument is a compile error.
- **Async** — built on [`reqwest`](https://docs.rs/reqwest); works on any Tokio runtime.
- **Modular** — every API is a Cargo feature, so you compile only what you use.
- **No magic** — generated code is committed; no build scripts, no proc-macros, no codegen at build time.

## Installation

```toml
[dependencies]
autogen-customerio = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Compile only the APIs you need (faster builds):

```toml
[dependencies]
autogen-customerio = { version = "0.1", default-features = false, features = ["track", "native-tls"] }
```

> **Note:** with `default-features = false` you must enable a TLS backend — either `native-tls`
> or `rustls` — or HTTPS requests will fail at runtime.

## Quick Start

```rust,no_run
use autogen_customerio::ids::DeliveryId;
use autogen_customerio::{AppApiKey, CustomerIoClient, Region};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CustomerIoClient::new(Region::Us);

    // Each accessor takes that API's credential and returns a ready `Configuration`.
    let app = client.app(&AppApiKey::new("your-app-api-key"));

    let message = autogen_customerio::app::apis::messages_api::get_message(
        &app,
        &DeliveryId::new("dgOq7wUB"),
        None,
    )
    .await?;
    println!("{message:?}");
    Ok(())
}
```

## The three APIs

| Feature | Module | Accessor | Auth | US base URL | EU base URL |
|---------|--------|----------|------|-------------|-------------|
| `track` | `track` | `client.track(&site_id, &api_key)` | Basic (Site ID + API key) | `https://track.customer.io` | `https://track-eu.customer.io` |
| `app` | `app` | `client.app(&app_api_key)` | Bearer (App API key) | `https://api.customer.io` | `https://api-eu.customer.io` |
| `pipelines` | `pipelines` | `client.pipelines(&api_key)` | Basic (source API key) | `https://cdp.customer.io/v1` | `https://cdp-eu.customer.io/v1` |

**TLS backends** (one required): `native-tls` (default) or `rustls`.

## Regions and authentication

Pick the region your workspace lives in once, with `CustomerIoClient::new(Region::Us)` or
`Region::Eu`. Each accessor takes its own credential type — `TrackSiteId` + `TrackApiKey`,
`AppApiKey`, `PipelinesApiKey` — so a key cannot be passed to the wrong API, and none of them
prints its value in `Debug` output.

To point at a proxy or mock server, mutate `base_path` on the returned configuration:

```rust,no_run
# use autogen_customerio::{CustomerIoClient, PipelinesApiKey, Region};
let client = CustomerIoClient::new(Region::Us);
let mut pipelines = client.pipelines(&PipelinesApiKey::new("key"));
pipelines.base_path = "http://localhost:8080".to_string();
```

## Typed IDs

`autogen_customerio::ids` holds a newtype per Customer.io entity. Generated function parameters,
model fields, and the `id` of entity models use them:

```rust,ignore
use autogen_customerio::app::apis::broadcasts_api;
use autogen_customerio::ids::{ActionId, BroadcastId};

let broadcast = BroadcastId::new(12);
let action = ActionId::new(34);
broadcasts_api::get_broadcast_action(&app, broadcast, action).await?; // ok
broadcasts_api::get_broadcast_action(&app, action, broadcast).await?; // compile error: swapped
```

Each ID serializes as the plain JSON number or string. Integer IDs are `i64`. An ID name the
allowlist does not cover yet stays a raw `i64` / `String`.

## Middleware

Every generated `Configuration.client` is a `reqwest_middleware::ClientWithMiddleware`. Use
`CustomerIoClient::builder` to attach middleware — for example a tracing, retry, or logging layer —
to every request the client makes:

```rust,no_run
use autogen_customerio::{CustomerIoClient, Region};

# fn example(my_middleware: impl autogen_customerio::reqwest_middleware::Middleware) {
let client = CustomerIoClient::builder(Region::Us)
    .with(my_middleware)
    .build();
# }
```

`autogen_customerio::reqwest_middleware` is re-exported so you build middleware against the same
version this crate links (reqwest 0.13 / reqwest-middleware 0.5; pair it with reqwest-tracing
0.7). The crate itself emits no spans, logs no URLs, and depends on no tracing crates.

## Error Handling

Generated functions return `Result<T, apis::Error<E>>`, where `E` is the endpoint-specific error
enum. Each API module has its own `apis::Error`, distinguishing transport errors,
(de)serialization errors, and structured API error responses:

```rust,ignore
use autogen_customerio::app::apis::Error;

match some_call(&config).await {
    Ok(resp) => { /* ... */ }
    Err(Error::ResponseError(e)) => eprintln!("API returned HTTP {}: {}", e.status, e.content),
    Err(Error::Reqwest(e)) => eprintln!("transport error: {e}"),
    Err(e) => eprintln!("other error: {e}"),
}
```

Enum fields accept values the spec does not list yet: each generated string enum has an
`UnknownValue(String)` variant, so one new value from the API does not fail the whole response.

## How It's Generated

```bash
# Requires only a JDK and python3 (the generator JAR is fetched automatically).
./generate.sh
```

`generate.sh` fetches the three specs from `https://docs.customer.io/files/`, records their
combined SHA-256 in `SPEC_HASH`, patches known spec defects, runs openapi-generator (rust +
reqwest template) on each, and vendors the generated `apis/` and `models/` into a per-API module
under `src/<api>/`. Then it promotes ID types, adds the open-enum fallback, and runs `cargo check`.

The specs need patching before generation. Each patch is a script in `scripts/` that documents
the defect and fails loudly on any shape it does not recognize:

- The specs are fully dereferenced, so inline copies of component schemas are turned back into
  `$ref`s (one Rust type per component instead of one per copy).
- Discriminated unions get an explicit tag mapping, so `{"action": "identify"}` serializes
  correctly instead of using a generated model name as the tag.
- App API Assets and Design Studio endpoints declare no security; they are defaulted to bearer
  auth.
- All integers are 64-bit, because Unix timestamps overflow `i32` in 2038.
- The inbound reporting-webhook payload schemas are not generated: no API operation uses them, and
  their tags are inconsistent upstream.

The generator version is **pinned** (`GENERATOR_VERSION` in `generate.sh`) and the JAR is
downloaded directly from Maven Central, so local and CI runs are byte-for-byte identical. The run
is idempotent: a fresh generation reproduces the committed tree exactly.

Only `Cargo.toml`, `src/lib.rs`, `src/client.rs`, `src/ids.rs`, `README.md`, and `CLAUDE.md` are
hand-written. Everything under `src/<api>/` is generated — **do not edit it by hand**.

### Staying in sync with the specs

A scheduled GitHub Action (`update-spec.yml`) regenerates from the live specs **daily**; when the
combined hash changes it bumps the patch version and opens a PR. Merging that PR tags the release
and publishes the new version to crates.io (`tag-release.yml`). Publishing requires a
`CARGO_REGISTRY_TOKEN` repository secret. The first `0.1.0` release is published manually
(`cargo publish`) to establish crate ownership.

## License

MIT
