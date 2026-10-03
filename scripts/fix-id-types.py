#!/usr/bin/env python3
"""Promote allowlisted ID parameters and fields to strict newtypes.

The Customer.io specs type every entity ID as a bare `integer` or `string`, so generated
functions take runs of adjacent `i64` / `&str` parameters where transposed arguments compile
fine. This pass rewrites a curated allowlist, in generated function signatures, `new()`
constructors, and model struct fields, to the hand-written newtypes in `src/ids.rs`
(crate::ids):

  - NAMED: `<name>_id` parameters and fields, per service. Track and App share one Journeys
    workspace, so they share the table; Pipelines has its own (its `user_id` is a string
    person identifier, while App's `user_id` is something else).
  - NAMED_PARAMS: names whose meaning is only certain for function parameters (App's
    `message_id` path parameter is a delivery ID).
  - ENTITY_IDS: the plain `id` field of an entity model (`BroadcastObject.id`), matched by
    model file. The generator named the campaign model `Segment` (its spec title), so
    `segment.rs` maps to CampaignId; the field docs were checked for every entry.

A name is only rewritten when its raw type matches the newtype's kind (integer IDs replace
`i64`, string IDs replace `String` / `&str`). The spec types `object_type_id` and
`parent_folder_id` as integer in some places and string in others, so they are not listed.

Deliberately allowlist-based: an unknown ID stays raw, so new spec fields never break
generation — they just arrive untyped until added here (keep src/ids.rs in sync).

Idempotent: a rewritten site no longer has a raw type and is skipped.

Usage: fix-id-types.py <src-dir>
"""
import re
import sys
from pathlib import Path

INT_TYPES = {
    "BroadcastId", "CampaignId", "NewsletterId", "ContentId", "ActionId", "TriggerId",
    "SegmentId", "SenderIdentityId", "SubscriptionTopicId", "TransactionalMessageId",
    "CollectionId", "ExportId", "ImportId",
}

JOURNEYS = {
    "broadcast_id": "BroadcastId",
    "campaign_id": "CampaignId",
    "newsletter_id": "NewsletterId",
    "content_id": "ContentId",
    "action_id": "ActionId",
    "parent_action_id": "ActionId",
    "multi_language_branch_action_id": "ActionId",
    "trigger_id": "TriggerId",
    "segment_id": "SegmentId",
    "sender_identity_id": "SenderIdentityId",
    "from_id": "SenderIdentityId",
    "reply_to_id": "SenderIdentityId",
    "subscription_topic_id": "SubscriptionTopicId",
    "transactional_id": "TransactionalMessageId",
    "transactional_message_id": "TransactionalMessageId",
    "collection_id": "CollectionId",
    "export_id": "ExportId",
    "warning_export_id": "ExportId",
    "error_export_id": "ExportId",
    "import_id": "ImportId",
    "delivery_id": "DeliveryId",
    "cio_id": "CioId",
    "anonymous_id": "AnonymousId",
}

NAMED = {
    "track": JOURNEYS,
    "app": JOURNEYS,
    "pipelines": {
        "anonymous_id": "AnonymousId",
        "user_id": "UserId",
    },
}

# Function parameters only (apis/*.rs). App message endpoints take the delivery ID as
# `message_id`; model fields named `message_id` (e.g. Track in-app events) are other IDs.
NAMED_PARAMS = {
    "app": {"message_id": "DeliveryId"},
}

# Model file stem -> newtype for its `id` field (Track and App).
ENTITY_IDS = {
    "broadcast_object": "BroadcastId",
    "segment": "CampaignId",
    "newsletter_object": "NewsletterId",
    "content_object": "ContentId",
    "email_action_object": "ActionId",
    "broadcast_trigger_object": "TriggerId",
    "segment_response_object": "SegmentId",
    "sender_identity_object": "SenderIdentityId",
    "get_topics_200_response_topics_inner": "SubscriptionTopicId",
    "transactional_object": "TransactionalMessageId",
    "collection_response": "CollectionId",
    "export_object": "ExportId",
    "import_object": "ImportId",
    "message_object": "DeliveryId",
    "archived_message_object": "DeliveryId",
}


def site_re(names: dict) -> re.Pattern:
    # `<name>: [Option<]*<raw>` in fn signatures, `new()` params, and `pub <name>:` fields.
    alternation = "|".join(sorted(names, key=len, reverse=True))
    return re.compile(r"\b(" + alternation + r"): ((?:Option<)*)(&str|String|i64)\b")


def rewrite(text: str, names: dict) -> tuple[str, int]:
    def repl(m: re.Match) -> str:
        name, options, raw = m.groups()
        rust_type = names[name]
        is_int = rust_type in INT_TYPES
        if is_int != (raw == "i64"):
            return m.group(0)  # spec types this name differently here; leave it raw
        prefix = "&" if raw == "&str" else ""
        return f"{name}: {options}{prefix}crate::ids::{rust_type}"

    return site_re(names).subn(repl, text)


def main() -> None:
    root = Path(sys.argv[1])
    files = sites = 0
    for service, names in NAMED.items():
        for path in sorted((root / service).glob("*/*.rs")):
            if path.parent.name not in ("apis", "models"):
                continue
            text = path.read_text()
            new_text, n = rewrite(text, names)
            if path.parent.name == "apis" and service in NAMED_PARAMS:
                new_text, m = rewrite(new_text, NAMED_PARAMS[service])
                n += m
            if service in ("track", "app") and path.parent.name == "models" and path.stem in ENTITY_IDS:
                new_text, m = rewrite(new_text, {"id": ENTITY_IDS[path.stem]})
                if m == 0 and "crate::ids::" + ENTITY_IDS[path.stem] not in new_text:
                    raise SystemExit(f"error: {path}: no `id` field to promote; review ENTITY_IDS")
                n += m
            if new_text != text:
                path.write_text(new_text)
                files += 1
                sites += n
    print(f"    promoted {sites} ID site(s) to strict newtypes across {files} file(s)")


if __name__ == "__main__":
    main()
