#!/usr/bin/env python3
"""Make discriminated unions generate a correct serde tagged enum.

The specs discriminate unions on a property (`discriminator: {propertyName: action}`) without
a `mapping`, and every leaf schema also declares that property as a required field. The rust
generator turns this into `#[serde(tag = "action")]` with two defects:

  - With no mapping it uses each member's *model name* as the tag, so `PersonOperations`
    serializes `{"action": "Person__Identify"}` instead of `{"action": "identify"}`.
  - The leaf struct keeps its own `action` field. Serialization writes the key twice, and
    deserialization fails because serde consumes the tag before the struct reads its
    required field.

Each leaf declares its tag as a one-value `enum` (or `const`), so the correct shape is
derivable. For every discriminated `oneOf`/`anyOf`, children first:

  - inline members are hoisted into `components/schemas` under their title (snake_cased),
    reusing an existing schema only when it is identical, and replaced by a `$ref`. When the
    title already names a different schema (e.g. an inline "Email" next to the `email`
    schema), the name is prefixed with the enclosing schema or operation
    (`update_content_request_email`);
  - each member's tag value is read from its leaves. A member that is itself a union (the App
    reporting webhook tags on `object_type`, then on `metric`) contributes the one value all
    its leaves share;
  - `mapping[<tag value>] = <$ref>` is filled in (an upstream mapping is kept as-is);
  - the tag property is removed from every leaf's `properties` and `required`, so serde
    alone owns it;
  - a discriminated `anyOf` becomes `oneOf` (the tag selects exactly one member, and the rust
    generator emits no variants for a discriminated `anyOf`).

Removing the tag from a leaf is only safe if the leaf is reached through unions alone. A
leaf `$ref`'d from anywhere else (a property, a request body) makes the script exit non-zero,
as does any other ambiguity: a member without a title, leaves that disagree on the tag
value, a duplicate tag, or a hoisted name already used by a different schema.

Runs on the downloaded spec files in the temp workdir, after SPEC_HASH is recorded (the hash
always reflects the raw upstream documents).

Usage: fix-discriminator-mapping.py <spec.json> [<spec.json> ...]
"""
import copy
import json
import re
import sys

REF_PREFIX = "#/components/schemas/"
UNIONS = ("oneOf", "anyOf")


def snake(title: str) -> str:
    split_camel = re.sub(r"([a-z0-9])([A-Z])", r"\1 \2", title)
    return "_".join(w.lower() for w in re.findall(r"[A-Za-z0-9]+", split_camel))


def resolve(schema: dict, schemas: dict) -> dict:
    if "$ref" in schema:
        return schemas[schema["$ref"].removeprefix(REF_PREFIX)]
    return schema


def leaves(schema: dict, schemas: dict, name=None) -> list[tuple]:
    """(component name or None, schema) for every non-union schema under `schema`."""
    if "$ref" in schema:
        name = schema["$ref"].removeprefix(REF_PREFIX)
        schema = schemas[name]
    for keyword in UNIONS:
        if keyword in schema:
            return [leaf for m in schema[keyword] for leaf in leaves(m, schemas)]
    return [(name, schema)]


def leaf_tag(leaf: dict, prop: str):
    """The leaf's single tag value, or None."""
    spec = leaf.get("properties", {}).get(prop)
    if spec is None:
        return None
    if "const" in spec:
        return spec["const"]
    values = spec.get("enum")
    if isinstance(values, list) and len(values) == 1:
        return values[0]
    return None


def hoist(member: dict, schemas: dict, pristine: dict, context: str, where: str) -> str:
    """Compare against `pristine` (schemas as they were before any tag was stripped), so a
    second copy of an already-hoisted member is recognized as the same schema."""
    if "title" not in member:
        raise SystemExit(f"error: {where}: inline discriminator member has no title")
    for name in (snake(member["title"]), snake(f"{context} {member['title']}")):
        if name not in schemas:
            pristine[name] = copy.deepcopy(member)
            schemas[name] = member
            return name
        if pristine.get(name) == member:
            return name
    raise SystemExit(f"error: {where}: no free schema name for inline member '{member['title']}'")


def strip_tag(leaf: dict, prop: str) -> None:
    leaf.get("properties", {}).pop(prop, None)
    if prop in leaf.get("required", []):
        leaf["required"] = [r for r in leaf["required"] if r != prop]
        if not leaf["required"]:
            del leaf["required"]


def rewrite(node: dict, schemas: dict, pristine: dict, stripped: set, context: str, where: str) -> int:
    keyword = next((k for k in UNIONS if k in node), None)
    if keyword is None:
        return 0  # allOf-style discriminator on a parent schema; not used by these specs
    disc = node["discriminator"]
    prop = disc["propertyName"]
    upstream_mapping = disc.get("mapping")

    mapping = {}
    members = []
    for member in node[keyword]:
        ref = member["$ref"] if "$ref" in member else REF_PREFIX + hoist(member, schemas, pristine, context, where)
        member_leaves = leaves({"$ref": ref}, schemas)
        # Read tags from the pristine copy: a leaf shared with an earlier union has already
        # had its tag stripped.
        tags = {leaf_tag(pristine.get(name, leaf), prop) for name, leaf in member_leaves}
        if len(tags) != 1 or None in tags:
            raise SystemExit(f"error: {where}: member {ref} has no single tag value for '{prop}' ({tags})")
        value = tags.pop()
        if value in mapping:
            raise SystemExit(f"error: {where}: duplicate tag value '{value}'")
        mapping[value] = ref
        members.append({"$ref": ref})
        for _, leaf in member_leaves:
            strip_tag(leaf, prop)
            stripped.add(id(leaf))

    if upstream_mapping is not None and upstream_mapping != mapping:
        raise SystemExit(f"error: {where}: upstream mapping {upstream_mapping} != derived {mapping}")
    del node[keyword]
    node["oneOf"] = members
    disc["mapping"] = mapping
    return 1


def walk(node, schemas: dict, pristine: dict, stripped: set, context: str, where: str) -> int:
    """`context` is the nearest enclosing component schema name or operationId."""
    count = 0
    if isinstance(node, dict):
        if isinstance(node.get("operationId"), str):
            context = node["operationId"]
        # Children first, so nested discriminated unions are hoisted before their parents.
        for key, value in list(node.items()):
            child_context = key if where == "#/components/schemas" else context
            count += walk(value, schemas, pristine, stripped, child_context, f"{where}/{key}")
        if isinstance(node.get("discriminator"), dict):
            count += rewrite(node, schemas, pristine, stripped, context, where)
    elif isinstance(node, list):
        for i, value in enumerate(node):
            count += walk(value, schemas, pristine, stripped, context, f"{where}/{i}")
    return count


def refs_outside_unions(node, names: set, found: list, in_union: bool = False) -> None:
    if isinstance(node, dict):
        ref = node.get("$ref")
        if isinstance(ref, str) and not in_union and ref.removeprefix(REF_PREFIX) in names:
            found.append(ref)
        for key, value in node.items():
            refs_outside_unions(value, names, found, key in UNIONS)
    elif isinstance(node, list):
        for value in node:
            refs_outside_unions(value, names, found, in_union)


def main() -> None:
    for path in sys.argv[1:]:
        with open(path) as f:
            spec = json.load(f)
        schemas = spec.setdefault("components", {}).setdefault("schemas", {})
        stripped: set[int] = set()
        pristine = copy.deepcopy(schemas)
        count = walk(spec, schemas, pristine, stripped, "", "#")

        names = {name for name, schema in schemas.items() if id(schema) in stripped}
        found: list[str] = []
        refs_outside_unions(spec, names, found)
        if found:
            raise SystemExit(f"error: {path}: tag stripped from schemas also used outside unions: {sorted(set(found))}")

        with open(path, "w") as f:
            json.dump(spec, f)
        print(f"    {path}: {count} discriminated union(s) fixed, tag removed from {len(names)} leaf schema(s)")


if __name__ == "__main__":
    main()
