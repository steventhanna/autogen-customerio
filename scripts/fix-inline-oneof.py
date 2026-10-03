#!/usr/bin/env python3
"""Rewrite inline primitive `oneOf` members that crash the rust generator.

The Customer.io specs are OpenAPI 3.1. For a `oneOf` member that has no `$ref` and no
`title`, RustClientCodegen names the enum variant from `schema.getType()`. The 3.1 parser
stores types in `types` instead, so `getType()` is null and generation aborts with a
NullPointerException ("Could not process model ..."). Two shapes occur:

  - `oneOf: [{type: T, ...}, {type: "null"}]` — a nullable scalar (852 times in the App
    spec, e.g. `broadcast_id`). Rewritten to the member itself with `type: [T, "null"]`, which
    the generator maps to `Option<T>` (or `Option<Option<T>>` when the field is optional,
    keeping "absent" and "null" distinct). An enum would be the wrong type here.
  - `oneOf: [{type: "string"}, {type: "integer"}, ...]` — a genuine scalar union (attribute
    values). Each member gets `title: <type>`, so the generator emits an untagged enum with
    `String(String)`, `Integer(i32)`, … variants: exactly what 3.0 specs produce.

Any other `oneOf` with an untitled inline scalar member is an unknown shape: the script
exits non-zero so an upstream change is reviewed instead of silently mis-typed.

Runs on the downloaded spec files in the temp workdir, after SPEC_HASH is recorded (the hash
always reflects the raw upstream documents).

Usage: fix-inline-oneof.py <spec.json> [<spec.json> ...]
"""
import json
import sys

SCALARS = {"string", "integer", "number", "boolean"}
# Keys a scalar member may carry besides `type` without changing its meaning for typing.
ANNOTATIONS = {"description", "example", "x-scalar-ignore", "format", "title"}


def is_inline_scalar(member) -> bool:
    return isinstance(member, dict) and "$ref" not in member and member.get("type") in SCALARS


def is_null(member) -> bool:
    return isinstance(member, dict) and member == {"type": "null"}


def rewrite(node: dict, counts: dict, where: str) -> None:
    members = node["oneOf"]
    if not any(is_inline_scalar(m) and "title" not in m for m in members):
        return

    for m in members:
        extra = set(m) - ANNOTATIONS - {"type"}
        if not is_null(m) and extra:
            raise SystemExit(f"error: {where}: oneOf member has unexpected keys {sorted(extra)}")

    if len(members) == 2 and sum(map(is_null, members)) == 1:
        scalar = next(m for m in members if not is_null(m))
        if not is_inline_scalar(scalar):
            raise SystemExit(f"error: {where}: nullable oneOf over a non-scalar member")
        del node["oneOf"]
        for key, value in scalar.items():
            node.setdefault(key, value)
        node["type"] = [scalar["type"], "null"]
        counts["nullable"] += 1
        return

    if all(is_inline_scalar(m) for m in members):
        types = [m["type"] for m in members]
        if len(set(types)) != len(types):
            raise SystemExit(f"error: {where}: scalar oneOf repeats a type {types}; titles would collide")
        for m in members:
            m.setdefault("title", m["type"])
        counts["union"] += 1
        return

    raise SystemExit(f"error: {where}: unrecognized oneOf shape {[m.get('type', '$ref') for m in members]}")


def walk(node, counts: dict, where: str) -> None:
    if isinstance(node, dict):
        if isinstance(node.get("oneOf"), list):
            rewrite(node, counts, where)
        for key, value in node.items():
            walk(value, counts, f"{where}/{key}")
    elif isinstance(node, list):
        for i, value in enumerate(node):
            walk(value, counts, f"{where}/{i}")


def main() -> None:
    for path in sys.argv[1:]:
        with open(path) as f:
            spec = json.load(f)
        counts = {"nullable": 0, "union": 0}
        walk(spec, counts, "#")
        with open(path, "w") as f:
            json.dump(spec, f)
        print(f"    {path}: {counts['nullable']} nullable scalar(s), {counts['union']} scalar union(s)")


if __name__ == "__main__":
    main()
