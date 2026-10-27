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

/// ConsentRecord : The data for a `consent` activity. The activity's `name` is the action, `opted_in` or `opted_out`. Customer.io fills in the optional fields when the change that created the record has that information. See [Consent records](/messaging/channels/subscriptions/consent-records/#what-a-record-contains).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// The channel the consent applies to, like `email` or `sms`. Absent when the consent applies to all of your messages.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// Where the change came from, like your subscription center or an SMS reply. The `subscription_preference_migration` and `sender_optout_migration` values mark records that Customer.io imported from preferences and opt-outs that existed before your account recorded consent.
    #[serde(rename = "collection_point", skip_serializing_if = "Option::is_none")]
    pub collection_point: Option<CollectionPoint>,
    /// The state the change left the person in.
    #[serde(rename = "consent_state", skip_serializing_if = "Option::is_none")]
    pub consent_state: Option<ConsentState>,
    /// The message the person responded to, like the email that contained the unsubscribe link.
    #[serde(rename = "delivery_id", skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<crate::ids::DeliveryId>,
    /// The sender's email address or phone number when the change happened.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// The identifier your request used to find the person.
    #[serde(rename = "profile_identifier", skip_serializing_if = "Option::is_none")]
    pub profile_identifier: Option<String>,
    /// The type of `profile_identifier`, like `email` or `id`.
    #[serde(rename = "profile_identifier_type", skip_serializing_if = "Option::is_none")]
    pub profile_identifier_type: Option<String>,
    /// The address or phone number the person's reply came from.
    #[serde(rename = "received_on", skip_serializing_if = "Option::is_none")]
    pub received_on: Option<String>,
    /// What the person sent back, like the body of an SMS reply.
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    /// The sender identity the consent applies to, if any.
    #[serde(rename = "sender_id", skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<i64>,
    /// The ID of the activity that produced this record, like the inbound SMS that contained STOP.
    #[serde(rename = "source_event_id", skip_serializing_if = "Option::is_none")]
    pub source_event_id: Option<String>,
    /// The subscription topic the consent applies to, if any.
    #[serde(rename = "topic_id", skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<i64>,
    /// The topic's name when the change happened. Renaming the topic later doesn't change this value.
    #[serde(rename = "topic_name", skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

impl ConsentRecord {
    /// The data for a `consent` activity. The activity's `name` is the action, `opted_in` or `opted_out`. Customer.io fills in the optional fields when the change that created the record has that information. See [Consent records](/messaging/channels/subscriptions/consent-records/#what-a-record-contains).
    pub fn new() -> ConsentRecord {
        ConsentRecord {
            channel: None,
            collection_point: None,
            consent_state: None,
            delivery_id: None,
            from: None,
            profile_identifier: None,
            profile_identifier_type: None,
            received_on: None,
            response: None,
            sender_id: None,
            source_event_id: None,
            topic_id: None,
            topic_name: None,
        }
    }
}
/// Where the change came from, like your subscription center or an SMS reply. The `subscription_preference_migration` and `sender_optout_migration` values mark records that Customer.io imported from preferences and opt-outs that existed before your account recorded consent.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CollectionPoint {
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "carrier_report")]
    CarrierReport,
    #[serde(rename = "double_opt_in_confirmation")]
    DoubleOptInConfirmation,
    #[serde(rename = "form")]
    Form,
    #[serde(rename = "import")]
    Import,
    #[serde(rename = "in_app_prompt")]
    InAppPrompt,
    #[serde(rename = "inbound_reply")]
    InboundReply,
    #[serde(rename = "list_unsubscribe_header")]
    ListUnsubscribeHeader,
    #[serde(rename = "one_click_unsubscribe")]
    OneClickUnsubscribe,
    #[serde(rename = "operator")]
    Operator,
    #[serde(rename = "sender_optout_migration")]
    SenderOptoutMigration,
    #[serde(rename = "subscription_center")]
    SubscriptionCenter,
    #[serde(rename = "subscription_preference_migration")]
    SubscriptionPreferenceMigration,
    #[serde(rename = "unsubscribe_link")]
    UnsubscribeLink,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for CollectionPoint {
    fn default() -> CollectionPoint {
        Self::Api
    }
}
/// The state the change left the person in.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConsentState {
    #[serde(rename = "subscribed")]
    Subscribed,
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for ConsentState {
    fn default() -> ConsentState {
        Self::Subscribed
    }
}

