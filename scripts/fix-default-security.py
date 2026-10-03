#!/usr/bin/env python3
"""Give a spec a document-level default security requirement before generation.

Customer.io's specs declare security per operation and set no document-level `security`.
Some operations omit it entirely — in the App API every Assets and Design Studio
endpoint — even though the API requires the same credential for them. The rust generator
attaches auth only where a requirement is declared, so those functions would send no
`Authorization` header and every call would fail with 401.

This pass sets the document-level `security` to the named scheme. Operations that declare
their own `security` (including an explicit `[]` for unauthenticated endpoints) keep it.
It refuses to run if the scheme is not defined, or if the spec already declares a
different document-level default, so an upstream change surfaces instead of being masked.

Runs on the downloaded spec files in the temp workdir, after SPEC_HASH is recorded (the hash
always reflects the raw upstream documents).

Usage: fix-default-security.py <spec.json> <scheme-name>
"""
import json
import sys


def main() -> None:
    path, scheme = sys.argv[1], sys.argv[2]
    with open(path) as f:
        spec = json.load(f)

    schemes = spec.get("components", {}).get("securitySchemes", {})
    if scheme not in schemes:
        raise SystemExit(f"error: {path} defines no security scheme '{scheme}' (has: {sorted(schemes)})")

    wanted = [{scheme: []}]
    existing = spec.get("security")
    if existing is not None and existing != wanted:
        raise SystemExit(f"error: {path} already declares document-level security {existing!r}")

    spec["security"] = wanted
    with open(path, "w") as f:
        json.dump(spec, f)
    print(f"    {path}: default security -> {scheme}")


if __name__ == "__main__":
    main()
