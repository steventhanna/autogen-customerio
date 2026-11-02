/*
 * Customer.io Track API
 *
 * # Overview  Our Track API provides ways to send real-time customer data to your Customer.io workspace including customer identification and event tracking.  # Use our Postman collection  We've generated a Postman collection to help you get started with our APIs.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  **NOTE**: Postman endpoints default to our US APIs. If you're in our European (EU) region, you'll need to add `-eu` to the server variables (`track_api_url` and `app_api_url`).  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-0f7ae1e8-8177-46fc-808a-2fd363dd52b9?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-0f7ae1e8-8177-46fc-808a-2fd363dd52b9%26entityType%3Dcollection%26workspaceId%3Db886877f-fc09-475f-84fe-6221a98f4d18#?env%5BCustomer.io%20API%20Environment%5D=W3sia2V5IjoidHJhY2tfYXBpX3VybCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiJ0cmFjay5jdXN0b21lci5pbyIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiYXBpLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzaXRlX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBpX2tleSIsInR5cGUiOiJzZWNyZXQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYmVhcmVyIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYnJvYWRjYXN0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaW1wb3J0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiZW1haWxfYWRkcmVzcyIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN1cHByZXNzaW9uX3R5cGUiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjb2xsZWN0aW9uX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoic25pcHBldF9uYW1lIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoid2ViaG9va19pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InNlbmRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImV4cG9ydF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6Im1lc3NhZ2VfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzZWdtZW50X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoibmV3c2xldHRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImNvbnRlbnRfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjYW1wYWlnbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImFjdGlvbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImN1c3RvbWVyX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoidHJhbnNhY3Rpb25hbF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRyaWdnZXJfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJmb3JtX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaWRlbnRpZmllciIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRldmljZV9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRlbGl2ZXJ5X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9XQ==)  # Server addresses: US and EU Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://track.customer.io | | EU | https://track-eu.customer.io |  Note that if your account is in the EU region and you send traffic to our US endpoints, we'll redirect it accordingly but this traffic still passes through US servers and data could be logged in the US.  # Authentication   Use a **private key** (`sk_`) or **public key** (`pk_`) as a bearer token: `Authorization: Bearer sk_us_...`. To route Track API data to destinations outside Customer.io, associate your public or private key with an integration source. Customer.io uses the integration source to determine where to send traffic from the key. If your workspace is configured to route Track API data to destinations outside Customer.io, the Track API rejects requests made with a key that has no integration source (`403 Forbidden`). When you create a key on the [API Credentials page](https://fly.customer.io/settings/api_credentials), choose where you'll use the key under **Used in**. Use a private key on your server and a public key in a browser or mobile app. [Learn more about public and private keys](/accounts/settings/managing-credentials/#public-and-private-keys).  **Legacy credentials:** If your integration uses a Site ID and Track API key, you can keep using them with Basic authentication. Each operation on this page lists both kinds of credentials. You can find your Site ID on the [API Credentials page](https://fly.customer.io/settings/api_credentials). You might also need it to set up in-app messaging in an older version of a mobile SDK.  # v1 vs v2 APIs  Most of the time, when we talk about *The Track API*, we're talking about the v1 API because the v2 API isn't used in any of our libraries and rarely used in libraries built by third parties; it's much more common that you'd encounter the v1 API.  If you're integrating with Customer.io using one of our libraries, or a third party customer data platform (CDP) like Segment or Rudderstack, you'll be using the v1 API.  The v2 API is newer and supports two important features that the v1 API doesn't natively support: objects and batching. But, if you're integrating directly with our API, we suggest you use the [Pipelines API](/integrations/api/cdp/). The Pipelines API supports both objects, batching, *and* all of our newest integrations and libraries are based on it.  # Rate Limits  The Track API has no fixed rate limit. Customer.io accepts your requests as fast as you send them and doesn't reject or drop them because of volume. Our general guideline is about 3000 requests per 3 seconds, an average of 1000 requests per second. The 3-second window leaves room for short bursts.  Traffic above this level usually comes from high-volume integrations, which our Premium and Enterprise plans support. If your traffic regularly goes above this, we may contact you to learn more about your integration and talk about optimizing it, or about a plan that better suits your volume.  **This guideline is subject to change.**  Below are the payload size limits for the Track API. If any of these limits are too restrictive for your needs, contact support to let us know your situation as we may be able to accommodate special circumstances.   ## Customer limits  These limits apply to people and their attributes, often referred to as \"customers\" in our APIs.  | Data Type | Limit | Description | | -- | -- | -- | | ID | 150 bytes | Max length of a person's ID value | | Attribute Name | 150 bytes | Max length of each attribute name | | Attribute Value | 1000 bytes | Max length of attribute values | | Unique attributes | 300 | Max number of attributes allowed per person or Identify call |  ## Object and relationship limits  Objects (groups) and relationships between people and objects can have their own attributes. Their limits are similar to people (customers).  | Data Type | Limit | Description | | -- | -- | -- | | Object ID | 150 bytes | Max length of a object's ID | | Attribute Names | 150 bytes | Max length of each attribute name | | Attribute Values | 1000 bytes | Max length of attribute values | | Unique attributes | 300 | Max number of attributes allowed per object or relationship | | Total attribute size | 100 Kilobytes | Max size of all attributes associated with an object or relationship |  ## Track API Event limits  These limits apply to events that you'll send with the `/v1/track` call.  | Data Type | Limit | Description | | -- | -- | -- | | Event Name | 100 bytes | Max length of each event name | | Event Data | 100000 bytes | Max length of each event data |   ## v2 API Limits  The v2 API has two endpoints, both of which have limits on the total size of requests.  * `/entity` is limited to requests 32kb or smaller. * `/batch` is limited to requests 500kb or smaller.      Each of the requests within a batch must also be 32kb or smaller. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::track::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewsletterObject {
    /// A list of message variants in a one-time send, where a variant is a translation or A/B test.
    #[serde(rename = "content_ids", skip_serializing_if = "Option::is_none")]
    pub content_ids: Option<Vec<i64>>,
    /// The date time when the referenced ID was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// An identifier in the format `id:timestamp` where the id is for the object you're working with (Automations, Deliveries, Exports, Identities, One-time sends, Segments, and Templates), and the timestamp is the last time the object was updated.
    #[serde(rename = "deduplicate_id", skip_serializing_if = "Option::is_none")]
    pub deduplicate_id: Option<String>,
    /// The send window in seconds, if you set one. Customer.io stops delivering the one-time send this long after it starts.
    #[serde(rename = "expires_after", skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<i64>,
    /// The identifier for a one-time send.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::ids::NewsletterId>,
    /// The name of the one-time send. Must be 190 characters or less.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Why the recipient file failed, when `recipient_file_status` is `error`. Customer.io omits it otherwise.
    #[serde(rename = "recipient_file_error", skip_serializing_if = "Option::is_none")]
    pub recipient_file_error: Option<String>,
    /// How far Customer.io has got with a recipient file, whether you uploaded it in the UI or passed `recipient_file_url`. Customer.io omits this field when the audience came from a `recipients` filter. Wait for `done` before you send or schedule. 
    #[serde(rename = "recipient_file_status", skip_serializing_if = "Option::is_none")]
    pub recipient_file_status: Option<RecipientFileStatus>,
    /// If the recipient conditions included segments, this returns a list of those segment ids.
    #[serde(rename = "recipient_segment_ids", skip_serializing_if = "Option::is_none")]
    pub recipient_segment_ids: Option<Vec<i64>>,
    /// When the one-time send is scheduled to go out, or `null` if it has no active schedule. Always included.  - A paused one-time send keeps its schedule, so this still has a value while `send_state` is `paused`. - After sending starts, this stores the next scheduled send if there is one, like the next timezone batch or the remainder of an A/B test. - When `tz_match_enabled` is `true`, the `scheduled_at` property is the time you asked for in the [schedule request](/integrations/api/app/tag/send-messages/scheduleNewsletter/) until sending starts. - If you send immediately instead of scheduling, there's a short delay before Customer.io starts sending. During that delay, `scheduled_at` isn't `null` and isn't a future time: it's the time you made the request, an earlier time, or `0`. 
    #[serde(rename = "scheduled_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<Option<i64>>,
    /// Where the one-time send is in its lifecycle. Always included. This matches the status shown in Customer.io, with one exception noted under `awaiting_winner`.  - `drafted`: The one-time send isn't scheduled and hasn't been sent. - `scheduled`: A send time is set and sending hasn't started. - `sending`: Sending has started and is still in progress. More batches are due (timezone-match batches or the remainder of an A/B test), or messages are still being delivered. - `sent`: The one-time send is no longer sending. This doesn't mean that every individual message has reached a final state. - `paused`: The one-time send is paused. - `stopped`: The one-time send was stopped. - `failed`: The one-time send failed. - `awaiting_winner`: An A/B test has started and is waiting for you to pick a winner before the rest of the audience gets it. Test messages might still be going out. Customer.io's UI stops showing \"awaiting winner\" 30 days after the first send, but the API keeps reporting `awaiting_winner` because you can still pick a winner.  When more than one state applies, Customer.io reports the first match in this order: `stopped`, `failed`, `paused`, `awaiting_winner`, `sending`, `sent`, `scheduled`, `drafted`. 
    #[serde(rename = "send_state", skip_serializing_if = "Option::is_none")]
    pub send_state: Option<SendState>,
    /// The last time the one-time send was sent, or `null` if it hasn't been sent.
    #[serde(rename = "sent_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<Option<i64>>,
    /// If you enabled a [subscription center](/journeys/channels/subscriptions/center/) on your workspace, this returns the id of the subscription preference you set.
    #[serde(rename = "subscription_topic_id", skip_serializing_if = "Option::is_none")]
    pub subscription_topic_id: Option<crate::ids::SubscriptionTopicId>,
    /// An array of tags associated with the one-time send.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The [IANA timezone name](/journeys/send/timezones/example-timezones/#region-format) of the schedule (for example, `America/New_York`). Customer.io includes this field only when the active schedule has a timezone. It's omitted otherwise. 
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::NewsletterMessageType>,
    /// `true` when the pending schedule sends at the same local time in each person's timezone. Customer.io includes this field only while a timezone-match schedule is pending. It's omitted otherwise and is never `false`. 
    #[serde(rename = "tz_match_enabled", skip_serializing_if = "Option::is_none")]
    pub tz_match_enabled: Option<bool>,
    /// The date time when the referenced ID was last updated.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
}

impl NewsletterObject {
    pub fn new() -> NewsletterObject {
        NewsletterObject {
            content_ids: None,
            created: None,
            deduplicate_id: None,
            expires_after: None,
            id: None,
            name: None,
            recipient_file_error: None,
            recipient_file_status: None,
            recipient_segment_ids: None,
            scheduled_at: None,
            send_state: None,
            sent_at: None,
            subscription_topic_id: None,
            tags: None,
            timezone: None,
            r#type: None,
            tz_match_enabled: None,
            updated: None,
        }
    }
}
/// How far Customer.io has got with a recipient file, whether you uploaded it in the UI or passed `recipient_file_url`. Customer.io omits this field when the audience came from a `recipients` filter. Wait for `done` before you send or schedule. 
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecipientFileStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "error")]
    Error,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for RecipientFileStatus {
    fn default() -> RecipientFileStatus {
        Self::Pending
    }
}
/// Where the one-time send is in its lifecycle. Always included. This matches the status shown in Customer.io, with one exception noted under `awaiting_winner`.  - `drafted`: The one-time send isn't scheduled and hasn't been sent. - `scheduled`: A send time is set and sending hasn't started. - `sending`: Sending has started and is still in progress. More batches are due (timezone-match batches or the remainder of an A/B test), or messages are still being delivered. - `sent`: The one-time send is no longer sending. This doesn't mean that every individual message has reached a final state. - `paused`: The one-time send is paused. - `stopped`: The one-time send was stopped. - `failed`: The one-time send failed. - `awaiting_winner`: An A/B test has started and is waiting for you to pick a winner before the rest of the audience gets it. Test messages might still be going out. Customer.io's UI stops showing \"awaiting winner\" 30 days after the first send, but the API keeps reporting `awaiting_winner` because you can still pick a winner.  When more than one state applies, Customer.io reports the first match in this order: `stopped`, `failed`, `paused`, `awaiting_winner`, `sending`, `sent`, `scheduled`, `drafted`. 
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SendState {
    #[serde(rename = "drafted")]
    Drafted,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "sending")]
    Sending,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "awaiting_winner")]
    AwaitingWinner,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for SendState {
    fn default() -> SendState {
        Self::Drafted
    }
}

