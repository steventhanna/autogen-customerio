#!/usr/bin/env python3
"""Reduce schema titles to letters, digits, and single spaces before generation.

openapi-generator names inline models after their `title`. The Customer.io specs use titles
with punctuation and trailing newlines ("SMS / MMS", "SMS/MMS", "Email: Properties\\n"). The
rust generator sanitizes such a name one way for the model file and another way for the
`pub mod` line in models/mod.rs, so the crate fails with "file not found for module
`sms_mms`". Keeping only alphanumeric words gives both sanitizers the same input; the
resulting Rust type names are unchanged (the generator camelizes words either way).

`info.title` is left alone. A property *named* `title` is a dict, not a string, so it is
never touched.

Runs on the downloaded spec files in the temp workdir, after SPEC_HASH is recorded (the hash
always reflects the raw upstream documents).

Usage: fix-schema-titles.py <spec.json> [<spec.json> ...]
"""
import json
import re
import sys


def clean(title: str) -> str:
    return " ".join(re.findall(r"[A-Za-z0-9]+", title))


def walk(node) -> int:
    count = 0
    if isinstance(node, dict):
        title = node.get("title")
        if isinstance(title, str) and clean(title) != title:
            node["title"] = clean(title)
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
        count = sum(walk(spec[key]) for key in ("paths", "components") if key in spec)
        with open(path, "w") as f:
            json.dump(spec, f)
        print(f"    {path}: {count} title(s) cleaned")


if __name__ == "__main__":
    main()
