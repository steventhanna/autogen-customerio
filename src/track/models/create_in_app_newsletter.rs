/*
 * Customer.io Track API
 *
 * # Overview  Our Track API provides ways to send real-time customer data to your Customer.io workspace including customer identification and event tracking.  # Use our Postman collection  We've generated a Postman collection to help you get started with our APIs.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  **NOTE**: Postman endpoints default to our US APIs. If you're in our European (EU) region, you'll need to add `-eu` to the server variables (`track_api_url` and `app_api_url`).  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-0f7ae1e8-8177-46fc-808a-2fd363dd52b9?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-0f7ae1e8-8177-46fc-808a-2fd363dd52b9%26entityType%3Dcollection%26workspaceId%3Db886877f-fc09-475f-84fe-6221a98f4d18#?env%5BCustomer.io%20API%20Environment%5D=W3sia2V5IjoidHJhY2tfYXBpX3VybCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiJ0cmFjay5jdXN0b21lci5pbyIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiYXBpLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzaXRlX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBpX2tleSIsInR5cGUiOiJzZWNyZXQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYmVhcmVyIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYnJvYWRjYXN0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaW1wb3J0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiZW1haWxfYWRkcmVzcyIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN1cHByZXNzaW9uX3R5cGUiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjb2xsZWN0aW9uX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoic25pcHBldF9uYW1lIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoid2ViaG9va19pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InNlbmRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImV4cG9ydF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6Im1lc3NhZ2VfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzZWdtZW50X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoibmV3c2xldHRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImNvbnRlbnRfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjYW1wYWlnbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImFjdGlvbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImN1c3RvbWVyX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoidHJhbnNhY3Rpb25hbF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRyaWdnZXJfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJmb3JtX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaWRlbnRpZmllciIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRldmljZV9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRlbGl2ZXJ5X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9XQ==)  # Server addresses: US and EU Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://track.customer.io | | EU | https://track-eu.customer.io |  Note that if your account is in the EU region and you send traffic to our US endpoints, we'll redirect it accordingly but this traffic still passes through US servers and data could be logged in the US.  # Authentication   You can find all of your API authentication information in your [Account Settings](https://fly.customer.io/settings/api_credentials). Our Tracking API uses HTTP basic authorization. The App API uses bearer authorization, and you can generate tokens supporting different scopes. Each operation in this document references the authorization header it requires.  # v1 vs v2 APIs  Most of the time, when we talk about *The Track API*, we're talking about the v1 API because the v2 API isn't used in any of our libraries and rarely used in libraries built by third parties; it's much more common that you'd encounter the v1 API.  If you're integrating with Customer.io using one of our libraries, or a third party customer data platform (CDP) like Segment or Rudderstack, you'll be using the v1 API.  The v2 API is newer and supports two important features that the v1 API doesn't natively support: objects and batching. But, if you're integrating directly with our API, we suggest you use the [Pipelines API](/integrations/api/cdp/). The Pipelines API supports both objects, batching, *and* all of our newest integrations and libraries are based on it.  # Rate Limits  The Track API has a rate limit of 1000 requests per second for both active data integrations and historical backfill scripts. This limit applies to both our v1 and v2 APIs.   While this rate is not strictly enforced, consistently exceeding it may lead to throttling or dropped data, especially during periods of high system load. If we detect a sustained high volume that could impact other customers, we may contact you to help adjust your integration or, in rare cases, temporarily block requests.  **Rate limits are subject to change. We may adjust these thresholds to ensure stable performance for all customers.**  Below are the payload size limits for the Track API. If any of these limits are too restrictive for your needs, contact support to let us know your situation as we may be able to accommodate special circumstances.   ## Customer limits  These limits apply to people and their attributes, often referred to as \"customers\" in our APIs.  | Data Type | Limit | Description | | -- | -- | -- | | ID | 150 bytes | Max length of a person's ID value | | Attribute Name | 150 bytes | Max length of each attribute name | | Attribute Value | 1000 bytes | Max length of attribute values | | Unique attributes | 300 | Max number of attributes allowed per person or Identify call |  ## Object and relationship limits  Objects (groups) and relationships between people and objects can have their own attributes. Their limits are similar to people (customers).  | Data Type | Limit | Description | | -- | -- | -- | | Object ID | 150 bytes | Max length of a object's ID | | Attribute Names | 150 bytes | Max length of each attribute name | | Attribute Values | 1000 bytes | Max length of attribute values | | Unique attributes | 300 | Max number of attributes allowed per object or relationship | | Total attribute size | 100 Kilobytes | Max size of all attributes associated with an object or relationship |  ## Track API Event limits  These limits apply to events that you'll send with the `/v1/track` call.  | Data Type | Limit | Description | | -- | -- | -- | | Event Name | 100 bytes | Max length of each event name | | Event Data | 100000 bytes | Max length of each event data |   ## v2 API Limits  The v2 API has two endpoints, both of which have limits on the total size of requests.  * `/entity` is limited to requests 32kb or smaller. * `/batch` is limited to requests 500kb or smaller.      Each of the requests within a batch must also be 32kb or smaller. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::track::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateInAppNewsletter {
    /// JSON body content for the in-app message, as a stringified JSON object—a JSON string, not a nested object. It supports these fields: - `message_id`—string, an identifier for the message - `carta_content`—string, the message content markup from our in-app editor - `display`—object; `mode` (`modal`, `overlay`, `inline`, or `tooltip`) and `value` (the position for modals: `top`, `center`, or `bottom`; the position ID for inline messages). Tooltips take a `tooltip_target` CSS selector instead of `value` - `priority`—integer, delivery order for queued messages: `1` (low), `5` (medium), or `10` (high) - `expiration`—object; if omitted, the message expires after 30 days (60 maximum). Set `type` (`relative` or `absolute`), `unit`, and `value` - `persistent`—boolean, keep the message visible across pages and sessions until your audience dismisses it - `attributes`—object, custom key-value string pairs delivered with the message - `route_rules` and `excluded_route_rules`—objects keyed by platform (`web`, `ios`, or `android`), each holding an array of [page rules](/messaging/channels/in-app/send-in-app-message/#page-rule) with an `operator` (`contains` or `equals`), a `value` (`*` is a wildcard), and an `order` (integer). You can only set page rules when you create a message, and platforms you don't set rules for won't show the message 
    #[serde(rename = "body_json")]
    pub body_json: String,
    /// The name of the one-time send.
    #[serde(rename = "name")]
    pub name: String,
    /// When `true`, Customer.io sends only one message per email address or phone number, even when several profiles share it. Supported on `email`, `twilio`, and `sms` one-time sends. Leave it out to use the default for your workspace's identifier configuration. No endpoint changes this later, so set it here if you don't want the default. 
    #[serde(rename = "deduped", skip_serializing_if = "Option::is_none")]
    pub deduped: Option<bool>,
    /// A send window in seconds, or `0` for no expiry. Customer.io stops delivering the one-time send once the window closes, so a delayed message doesn't arrive when it's no longer useful. Set it here only alongside `send_now` or `scheduled_at`, because the window runs from the send. On a plain draft, set it on the [send](/integrations/api/app/tag/send-messages/sendNewsletter/) or [schedule](/integrations/api/app/tag/send-messages/scheduleNewsletter/) call instead. 
    #[serde(rename = "expires_after", skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<i64>,
    /// Maximum number of messages per time period. **NOTE:** Though this states `email_rate`, you can use this for other message channels. Only fixed rate limits are supported, not [daily ramp limits](/messaging/send/broadcasts/one-time-sends/#daily-ramp). 
    #[serde(rename = "rate_limit_email_rate", skip_serializing_if = "Option::is_none")]
    pub rate_limit_email_rate: Option<i64>,
    /// When `true`, spreads messages evenly across the time period. Otherwise, it sends as fast as possible up to the limit in each period.
    #[serde(rename = "rate_limit_spread", skip_serializing_if = "Option::is_none")]
    pub rate_limit_spread: Option<bool>,
    /// Time period in seconds for rate limiting. Must be one of 60 (minute), 3600 (hour), or 86400 (day).
    #[serde(rename = "rate_limit_time_period", skip_serializing_if = "Option::is_none")]
    pub rate_limit_time_period: Option<RateLimitTimePeriod>,
    /// A URL pointing to a CSV of recipients, used instead of the `recipients` filter. Customer.io resolves the file in the background, so this request can't also send or schedule. Poll [Get a one-time send](/integrations/api/app/tag/newsletters/getNewsletters/) until `recipient_file_status` is `done`, then call [send](/integrations/api/app/tag/send-messages/sendNewsletter/) or [schedule](/integrations/api/app/tag/send-messages/scheduleNewsletter/). 
    #[serde(rename = "recipient_file_url", skip_serializing_if = "Option::is_none")]
    pub recipient_file_url: Option<String>,
    #[serde(rename = "recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Box<models::Email1Recipients>>,
    /// A Unix timestamp indicating when to send the one-time send. The time must be in the future. You cannot set both `send_now` and `scheduled_at`.
    #[serde(rename = "scheduled_at", skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<i64>,
    /// If `true`, the one-time send is sent immediately after creation. You cannot set both `send_now` and `scheduled_at`.
    #[serde(rename = "send_now", skip_serializing_if = "Option::is_none")]
    pub send_now: Option<bool>,
    /// The one-time send audience's subscription topic ID. Required when [subscription center](/journeys/channels/subscriptions/center/#enable-sub-center) is enabled; otherwise sends to globally subscribed profiles.
    #[serde(rename = "subscription_topic_id", skip_serializing_if = "Option::is_none")]
    pub subscription_topic_id: Option<crate::ids::SubscriptionTopicId>,
    /// The IDs of the tags you want to apply to the one-time send. Each tag must already exist.
    #[serde(rename = "tag_ids", skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<i64>>,
}

impl CreateInAppNewsletter {
    pub fn new(body_json: String, name: String) -> CreateInAppNewsletter {
        CreateInAppNewsletter {
            body_json,
            name,
            deduped: None,
            expires_after: None,
            rate_limit_email_rate: None,
            rate_limit_spread: None,
            rate_limit_time_period: None,
            recipient_file_url: None,
            recipients: None,
            scheduled_at: None,
            send_now: None,
            subscription_topic_id: None,
            tag_ids: None,
        }
    }
}
/// Time period in seconds for rate limiting. Must be one of 60 (minute), 3600 (hour), or 86400 (day).
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RateLimitTimePeriod {
    #[serde(rename = "60")]
    Variant60,
    #[serde(rename = "3600")]
    Variant3600,
    #[serde(rename = "86400")]
    Variant86400,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for RateLimitTimePeriod {
    fn default() -> RateLimitTimePeriod {
        Self::Variant60
    }
}

