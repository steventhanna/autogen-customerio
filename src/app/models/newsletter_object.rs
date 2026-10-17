/*
 * Customer.io App API
 *
 * Our App API provides ways to trigger messages and retrieve information about people, automations, broadcasts, and more.  # Overview  The App API provides methods to send newsletters, transactional messages, and API-triggered broadcasts. You can create newsletters from scratch and update transactional messages and API-triggered broadcasts.  For transactional messages and API-triggered broadcasts, your payload acts as a message \"trigger\" and can contain `data` that you reference in your messages using liquid—`{{trigger.<data>}}`.  The other endpoints help you retrieve information about people, segments, automations, broadcasts, etc; it also lets you update automation actions, messages, newsletter variants, etc. Aside from the [API-triggered broadcast](/integrations/api/app/tag/send-messages/triggerBroadcast/) endpoint (1 per 10 seconds), requests are limited to 10 per second. [Transactional](/integrations/api/app/tag/send-messages/sendEmail/) messages aren't rate limited.  # We may add to these APIs  As we release new features, we add endpoints and we may even add fields to existing endpoints. We don't announce these additions in advance. Fields we add will be backward compatible: existing fields keep their names and types.  Build your integration to ignore fields it doesn't recognize. If you validate responses against a schema, make sure the schema accepts unknown keys. In JSON Schema, for example, don't set `additionalProperties: false`.  # Use our Postman collection  We've generated a Postman collection to help you get started with our APIs.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  **NOTE**: Postman endpoints default to our US APIs. If you're in our European (EU) region, you'll need to add `-eu` to the server variables (`track_api_url` and `app_api_url`).  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d%26entityType%3Dcollection%26workspaceId%3Db886877f-fc09-475f-84fe-6221a98f4d18#?env%5BCustomer.io%20API%20Environment%5D=W3sia2V5IjoidHJhY2tfYXBpX3VybCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiJ0cmFjay5jdXN0b21lci5pbyIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiYXBpLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzaXRlX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBpX2tleSIsInR5cGUiOiJzZWNyZXQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYmVhcmVyIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYnJvYWRjYXN0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaW1wb3J0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiZW1haWxfYWRkcmVzcyIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN1cHByZXNzaW9uX3R5cGUiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjb2xsZWN0aW9uX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoic25pcHBldF9uYW1lIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoid2ViaG9va19pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InNlbmRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImV4cG9ydF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6Im1lc3NhZ2VfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzZWdtZW50X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoibmV3c2xldHRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImNvbnRlbnRfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjYW1wYWlnbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImFjdGlvbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImN1c3RvbWVyX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoidHJhbnNhY3Rpb25hbF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRyaWdnZXJfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJmb3JtX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaWRlbnRpZmllciIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRldmljZV9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRlbGl2ZXJ5X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9XQ==)  # Server addresses: US and EU  Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://api.customer.io | | EU | https://api-eu.customer.io |  # Authentication  All requests to the Customer.io App API use an [App API Key](#authentication).  To authenticate, provide your key as a Bearer token in a HTTP Authorization header. You can create and manage your API keys—including keys with different scopes—in [your account settings page](https://fly.customer.io/settings/api_credentials?keyType=app). Each operation on this page references the authorization header it requires.  # Rate Limits  Most endpoints on this page are limited to 10 requests per second. The exceptions are: * [Transactional messages](/integrations/api/app/tag/send-messages/sendEmail/) aren't rate limited. * The [API-triggered broadcast endpoint](/integrations/api/app/tag/send-messages/triggerBroadcast/) is limited to 1 request every 10 seconds. If you need to trigger broadcasts more often, contact us to talk through your use case.  **Rate limits are subject to change. We may adjust these thresholds to ensure stable performance for all customers.** 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::app::models;
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

