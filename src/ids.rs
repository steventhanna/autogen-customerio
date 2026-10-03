//! Hand-written strict ID types shared by the generated API modules.
//!
//! The Customer.io specs type every entity ID as a bare `integer` or `string`, so generated
//! functions take runs of adjacent `i64` parameters (`broadcast_id`, then `action_id`, …)
//! where a transposed argument compiles fine. `scripts/fix-id-types.py` rewrites a curated
//! allowlist of ID parameters and model fields to these newtypes after generation, so passing
//! a [`CampaignId`] where a [`BroadcastId`] belongs is a compile error. Spec names that alias
//! one entity (`from_id`, `reply_to_id` → [`SenderIdentityId`]) map to the same newtype, and
//! the `id` field of an entity model (`BroadcastObject.id`) gets that entity's type, so an ID
//! read from one response can be passed straight into the next call.
//!
//! The Track and App APIs share one Journeys workspace, so their IDs share these types. IDs not
//! on the allowlist stay raw; extend the tables in `scripts/fix-id-types.py` (and this file) to
//! promote more.
//!
//! Every type serializes as the plain JSON number or string. Construct with `new`, read with
//! `get` / `as_str`.

macro_rules! int_id {
    ($(#[doc = $doc:expr])* $name:ident) => {
        $(#[doc = $doc])*
        // `Default` is required because the generated models derive it; a zero ID is never
        // sent unless a caller builds a model with `Default::default()`.
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default,
            serde::Serialize, serde::Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(i64);

        impl $name {
            #[doc = concat!("Wrap a raw Customer.io ID as a [`", stringify!($name), "`].")]
            pub const fn new(value: i64) -> Self {
                Self(value)
            }

            /// The raw numeric ID.
            pub const fn get(self) -> i64 {
                self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<$name> for i64 {
            fn from(value: $name) -> i64 {
                value.0
            }
        }
    };
}

macro_rules! string_id {
    ($(#[doc = $doc:expr])* $name:ident) => {
        $(#[doc = $doc])*
        // `Default` is required because the generated models derive it.
        #[derive(
            Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default,
            serde::Serialize, serde::Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Wrap a raw Customer.io ID as a [`", stringify!($name), "`].")]
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            /// The raw string ID.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        // The generated code URL-encodes path parameters through `AsRef<str>`.
        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> String {
                value.0
            }
        }
    };
}

int_id! {
    /// A broadcast (API-triggered broadcast).
    BroadcastId
}
int_id! {
    /// A campaign (automation).
    CampaignId
}
int_id! {
    /// A newsletter (one-time send).
    NewsletterId
}
int_id! {
    /// A newsletter content variant: a language or A/B test variant of a newsletter.
    ContentId
}
int_id! {
    /// An action (a message or step) in a campaign, broadcast, or newsletter. Also used for
    /// `parent_action_id` and `multi_language_branch_action_id`.
    ActionId
}
int_id! {
    /// A broadcast trigger (one triggering of an API-triggered broadcast).
    TriggerId
}
int_id! {
    /// A segment.
    SegmentId
}
int_id! {
    /// A sender identity. Also used for `from_id` and `reply_to_id`, which reference one.
    SenderIdentityId
}
int_id! {
    /// A subscription topic.
    SubscriptionTopicId
}
int_id! {
    /// A transactional message. Also used for `transactional_id`.
    TransactionalMessageId
}
int_id! {
    /// A collection.
    CollectionId
}
int_id! {
    /// An export. Also used for `warning_export_id` and `error_export_id`.
    ExportId
}
int_id! {
    /// An import.
    ImportId
}
string_id! {
    /// A delivery: one message instance sent to one recipient.
    DeliveryId
}
string_id! {
    /// A person's Customer.io-assigned canonical ID (`cio_id`).
    CioId
}
string_id! {
    /// An anonymous visitor ID, such as a cookie value (Track and Pipelines).
    AnonymousId
}
string_id! {
    /// A Data Pipelines `userId`: your identifier for a known person.
    UserId
}
