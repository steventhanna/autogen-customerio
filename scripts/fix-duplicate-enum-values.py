#!/usr/bin/env python3
"""Drop repeated values from `enum` lists before generation.

JSON Schema expects enum values to be unique, but some Customer.io schemas repeat one (e.g.
`center` twice in the widget cross-axis alignment). The rust generator emits a second variant
(`Center2`) with the same serde name, which can never be deserialized into and triggers
`unreachable_patterns` warnings. Keeping the first occurrence changes no accepted value.

Runs on the downloaded spec files in the temp workdir, after SPEC_HASH is recorded (the hash
always reflects the raw upstream documents).

Usage: fix-duplicate-enum-values.py <spec.json> [<spec.json> ...]
"""
import json
import sys


def walk(node) -> int:
    count = 0
    if isinstance(node, dict):
        values = node.get("enum")
        if isinstance(values, list):
            unique = []
            for v in values:
                if v not in unique:
                    unique.append(v)
            if len(unique) != len(values):
                count += len(values) - len(unique)
                node["enum"] = unique
        for value in node.values():
            count += walk(value)
    elif isinstance(node, list):
        for value in node:
            count += walk(value)
    return count


def main() -> None:
    for path in sys.argv[1:]:
        with open(path) as f:
            spec = json.load(f)
        count = walk(spec)
        with open(path, "w") as f:
            json.dump(spec, f)
        print(f"    {path}: {count} duplicate enum value(s) dropped")


if __name__ == "__main__":
    main()
