#!/usr/bin/env python3
"""Remove the inbound reporting-webhook payload models from the App spec before generation.

The App spec documents Customer.io's *outbound* reporting webhooks (the payloads Customer.io
POSTs to you) in its `webhooks` section and in the `reporting_webhook` / `*_event` component
schemas. No API operation uses them, and they cannot be generated correctly: the outer union
tags on `object_type`, but the Customer leaves list all eight `object_type` values, one leaf
spells the field `object_type_type`, and the channel leaves are `allOf` compositions with no
tag of their own. Generated as-is, the enums would carry made-up tag values
("Email__drafted"). This crate is an API client, so the payload models are dropped rather
than shipped wrong.

The names are explicit: if one disappears or is renamed upstream the script exits non-zero,
so the list is reviewed instead of silently going stale.

Runs on the downloaded spec file in the temp workdir, after SPEC_HASH is recorded (the hash
always reflects the raw upstream documents).

Usage: drop-webhook-schemas.py <spec.json>
"""
import json
import sys

WEBHOOK_SCHEMAS = [
    "reporting_webhook",
    "email_event",
    "push_event",
    "sms_event",
    "whatsapp_event",
    "slack_event",
    "webhook_event",
    "in_app_event",
]


def main() -> None:
    path = sys.argv[1]
    with open(path) as f:
        spec = json.load(f)

    schemas = spec["components"]["schemas"]
    missing = [name for name in WEBHOOK_SCHEMAS if name not in schemas]
    if missing:
        raise SystemExit(f"error: {path}: expected webhook schemas not found: {missing}")
    if "webhooks" not in spec:
        raise SystemExit(f"error: {path}: no `webhooks` section; review WEBHOOK_SCHEMAS")

    for name in WEBHOOK_SCHEMAS:
        del schemas[name]
    del spec["webhooks"]

    with open(path, "w") as f:
        json.dump(spec, f)
    print(f"    {path}: dropped `webhooks` and {len(WEBHOOK_SCHEMAS)} reporting-webhook schema(s)")


if __name__ == "__main__":
    main()
