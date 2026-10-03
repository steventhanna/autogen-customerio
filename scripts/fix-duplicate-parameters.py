#!/usr/bin/env python3
"""Drop verbatim-duplicate parameters before generation.

OpenAPI requires each parameter list to be unique by (name, in). The App spec repeats the
whole list on `/v1/campaigns/{broadcast_id}/triggers/{trigger_id}/errors`, and the rust
generator emits a function with duplicate argument names, which does not compile. This pass
keeps the first of each identical pair. Two parameters with the same (name, in) but a
different definition are an upstream conflict with no safe choice, so the script exits
non-zero for them.

Runs on the downloaded spec files in the temp workdir, after SPEC_HASH is recorded (the hash
always reflects the raw upstream documents).

Usage: fix-duplicate-parameters.py <spec.json> [<spec.json> ...]
"""
import json
import sys

METHODS = {"get", "put", "post", "delete", "options", "head", "patch", "trace"}


def dedupe(params: list, where: str) -> tuple[list, int]:
    seen = {}
    kept = []
    for p in params:
        if "$ref" in p:
            kept.append(p)
            continue
        key = (p["name"], p["in"])
        if key in seen:
            if seen[key] != p:
                raise SystemExit(f"error: {where}: conflicting definitions of parameter {key}")
            continue
        seen[key] = p
        kept.append(p)
    return kept, len(params) - len(kept)


def main() -> None:
    for path in sys.argv[1:]:
        with open(path) as f:
            spec = json.load(f)
        dropped = 0
        for route, item in spec.get("paths", {}).items():
            holders = [("", item)] + [(m, op) for m, op in item.items() if m in METHODS]
            for method, holder in holders:
                if isinstance(holder.get("parameters"), list):
                    holder["parameters"], n = dedupe(holder["parameters"], f"{method} {route}")
                    dropped += n
        with open(path, "w") as f:
            json.dump(spec, f)
        print(f"    {path}: {dropped} duplicate parameter(s) dropped")


if __name__ == "__main__":
    main()
