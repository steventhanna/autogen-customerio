#!/usr/bin/env python3
"""Replace inline copies of component schemas with `$ref`s before generation.

The Customer.io specs are fully dereferenced: every operation and every component carries its
own inline copy of each schema it uses, and no `$ref` remains. openapi-generator then emits
one model per copy (`PersonOperations`, `PersonOperations1`, `SmsAttempted1`, …), so the same
API type has several unrelated Rust types and the names depend on traversal order.

This pass restores the references. Walking top-down, any inline schema that is identical to a
component schema is replaced by `{"$ref": "#/components/schemas/<name>"}` and not descended
into. Inside `components/schemas` each component's own root is kept; only its children are
rewritten. Only structured schemas are matched (objects with `properties`, unions, `allOf`,
and enums), so a bare `{"type": "string"}` never turns into a reference. When two components
are identical, the first one in document order is the target.

Runs on the downloaded spec files in the temp workdir, after SPEC_HASH is recorded (the hash
always reflects the raw upstream documents).

Usage: rereference-schemas.py <spec.json> [<spec.json> ...]
"""
import json
import sys

REF_PREFIX = "#/components/schemas/"
STRUCTURED = ("properties", "oneOf", "anyOf", "allOf", "enum")


def key(schema) -> str:
    return json.dumps(schema, sort_keys=True)


def is_structured(node) -> bool:
    return isinstance(node, dict) and "$ref" not in node and any(k in node for k in STRUCTURED)


def replace(node, index: dict, count: list) -> object:
    if isinstance(node, dict):
        if is_structured(node):
            target = index.get(key(node))
            if target is not None:
                count[0] += 1
                return {"$ref": REF_PREFIX + target}
        return {k: replace(v, index, count) for k, v in node.items()}
    if isinstance(node, list):
        return [replace(v, index, count) for v in node]
    return node


def main() -> None:
    for path in sys.argv[1:]:
        with open(path) as f:
            spec = json.load(f)
        schemas = spec.get("components", {}).get("schemas", {})

        index: dict[str, str] = {}
        for name, schema in schemas.items():
            if is_structured(schema):
                index.setdefault(key(schema), name)

        count = [0]
        # Index from the original components, then rewrite each component's children and
        # every other part of the document against that fixed index.
        for name, schema in schemas.items():
            schemas[name] = {k: replace(v, index, count) for k, v in schema.items()} if isinstance(schema, dict) else schema
        for section in list(spec):
            if section != "components":
                spec[section] = replace(spec[section], index, count)

        with open(path, "w") as f:
            json.dump(spec, f)
        print(f"    {path}: {count[0]} inline schema copy(ies) replaced by $ref")


if __name__ == "__main__":
    main()
