#!/usr/bin/env python3
"""Make every `integer` schema `format: int64` before generation.

The rust generator hard-codes `i32` for an `integer` with no `format` (type mappings do not
apply there), and the specs also mark some Unix timestamps (`created`, `first_started`) as
`int32`. Customer.io sends timestamps and numeric IDs as plain integers, so `i32` overflows
on 2038-01-19 and on large IDs. Widening to `int64` accepts every value `int32` did.

Covers `type: integer` and the 3.1 form `type: ["integer", "null"]`.

Runs on the downloaded spec files in the temp workdir, after SPEC_HASH is recorded (the hash
always reflects the raw upstream documents).

Usage: fix-integer-format.py <spec.json> [<spec.json> ...]
"""
import json
import sys


def is_integer(node: dict) -> bool:
    t = node.get("type")
    return t == "integer" or (isinstance(t, list) and "integer" in t)


def walk(node) -> int:
    count = 0
    if isinstance(node, dict):
        if is_integer(node) and node.get("format") != "int64":
            node["format"] = "int64"
            count += 1
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
        print(f"    {path}: {count} integer(s) set to int64")


if __name__ == "__main__":
    main()
