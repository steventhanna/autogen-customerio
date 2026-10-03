/*
 * Customer.io App API
 *
 * Our App API provides ways to trigger messages and retrieve information about people, automations, broadcasts, and more.  # Overview  The App API provides methods to send newsletters, transactional messages, and API-triggered broadcasts. You can create newsletters from scratch and update transactional messages and API-triggered broadcasts.  For transactional messages and API-triggered broadcasts, your payload acts as a message \"trigger\" and can contain `data` that you reference in your messages using liquid—`{{trigger.<data>}}`.  The other endpoints help you retrieve information about people, segments, automations, broadcasts, etc; it also lets you update automation actions, messages, newsletter variants, etc. Aside from the [API-triggered broadcast](/integrations/api/app/tag/send-messages/triggerBroadcast/) endpoint (1 per 10 seconds), requests are limited to 10 per second. [Transactional](/integrations/api/app/tag/send-messages/sendEmail/) messages go through the same high-throughput ingress as our Track and Pipelines APIs and share their soft rate limit of 3000 requests per 3 seconds, which isn't strictly enforced.  # We may add to these APIs  As we release new features, we add endpoints and we may even add fields to existing endpoints. We don't announce these additions in advance. Fields we add will be backward compatible: existing fields keep their names and types.  Build your integration to ignore fields it doesn't recognize. If you validate responses against a schema, make sure the schema accepts unknown keys. In JSON Schema, for example, don't set `additionalProperties: false`.  # Use our Postman collection  We've generated a Postman collection to help you get started with our APIs.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  **NOTE**: Postman endpoints default to our US APIs. If you're in our European (EU) region, you'll need to add `-eu` to the server variables (`track_api_url` and `app_api_url`).  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d%26entityType%3Dcollection%26workspaceId%3Db886877f-fc09-475f-84fe-6221a98f4d18#?env%5BCustomer.io%20API%20Environment%5D=W3sia2V5IjoidHJhY2tfYXBpX3VybCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiJ0cmFjay5jdXN0b21lci5pbyIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiYXBpLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzaXRlX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBpX2tleSIsInR5cGUiOiJzZWNyZXQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYmVhcmVyIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYnJvYWRjYXN0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaW1wb3J0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiZW1haWxfYWRkcmVzcyIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN1cHByZXNzaW9uX3R5cGUiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjb2xsZWN0aW9uX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoic25pcHBldF9uYW1lIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoid2ViaG9va19pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InNlbmRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImV4cG9ydF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6Im1lc3NhZ2VfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzZWdtZW50X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoibmV3c2xldHRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImNvbnRlbnRfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjYW1wYWlnbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImFjdGlvbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImN1c3RvbWVyX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoidHJhbnNhY3Rpb25hbF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRyaWdnZXJfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJmb3JtX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaWRlbnRpZmllciIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRldmljZV9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRlbGl2ZXJ5X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9XQ==)  # Server addresses: US and EU  Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://api.customer.io | | EU | https://api-eu.customer.io |  # Authentication  All requests to the Customer.io App API use an [App API Key](#authentication).  To authenticate, provide your key as a Bearer token in a HTTP Authorization header. You can create and manage your API keys—including keys with different scopes—in [your account settings page](https://fly.customer.io/settings/api_credentials?keyType=app). Each operation on this page references the authorization header it requires.  # Rate Limits  Most endpoints on this page are limited to 10 requests per second. The exceptions are: * [Transactional messages](/integrations/api/app/tag/send-messages/sendEmail/) use the same soft rate limit of our Track and Pipelines APIs (3000 requests per 3 seconds, not strictly enforced). * The [API-triggered broadcast endpoint](/integrations/api/app/tag/send-messages/triggerBroadcast/) is limited to 1 request every 10 seconds.  **Rate limits are subject to change. We may adjust these thresholds to ensure stable performance for all customers.** 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::app::models;
use serde::{Deserialize, Serialize};

/// ActivityTypes : The type of activity. Types with `_o:<object_type_id>` are for objects and types with `_r:<object_type_id>` are for relationships.
/// The type of activity. Types with `_o:<object_type_id>` are for objects and types with `_r:<object_type_id>` are for relationships.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActivityTypes {
    #[serde(rename = "add_relationship")]
    AddRelationship,
    #[serde(rename = "anon_merge")]
    AnonMerge,
    #[serde(rename = "attempted_action")]
    AttemptedAction,
    #[serde(rename = "attempted_email")]
    AttemptedEmail,
    #[serde(rename = "attempted_in_app")]
    AttemptedInApp,
    #[serde(rename = "attempted_push")]
    AttemptedPush,
    #[serde(rename = "attempted_slack")]
    AttemptedSlack,
    #[serde(rename = "attempted_twilio")]
    AttemptedTwilio,
    #[serde(rename = "attempted_webhook")]
    AttemptedWebhook,
    #[serde(rename = "attempted_whatsapp")]
    AttemptedWhatsapp,
    #[serde(rename = "attribute_change")]
    AttributeChange,
    #[serde(rename = "bounced_action")]
    BouncedAction,
    #[serde(rename = "bounced_email")]
    BouncedEmail,
    #[serde(rename = "bounced_push")]
    BouncedPush,
    #[serde(rename = "bounced_twilio")]
    BouncedTwilio,
    #[serde(rename = "bounced_whatsapp")]
    BouncedWhatsapp,
    #[serde(rename = "clicked_action")]
    ClickedAction,
    #[serde(rename = "clicked_content")]
    ClickedContent,
    #[serde(rename = "clicked_email")]
    ClickedEmail,
    #[serde(rename = "clicked_in_app")]
    ClickedInApp,
    #[serde(rename = "clicked_push")]
    ClickedPush,
    #[serde(rename = "clicked_twilio")]
    ClickedTwilio,
    #[serde(rename = "clicked_webhook")]
    ClickedWebhook,
    #[serde(rename = "converted_action")]
    ConvertedAction,
    #[serde(rename = "converted_content")]
    ConvertedContent,
    #[serde(rename = "converted_email")]
    ConvertedEmail,
    #[serde(rename = "converted_in_app")]
    ConvertedInApp,
    #[serde(rename = "converted_slack")]
    ConvertedSlack,
    #[serde(rename = "converted_twilio")]
    ConvertedTwilio,
    #[serde(rename = "converted_webhook")]
    ConvertedWebhook,
    #[serde(rename = "converted_whatsapp")]
    ConvertedWhatsapp,
    #[serde(rename = "deferred_action")]
    DeferredAction,
    #[serde(rename = "deferred_email")]
    DeferredEmail,
    #[serde(rename = "deferred_in_app")]
    DeferredInApp,
    #[serde(rename = "deferred_push")]
    DeferredPush,
    #[serde(rename = "deferred_slack")]
    DeferredSlack,
    #[serde(rename = "deferred_twilio")]
    DeferredTwilio,
    #[serde(rename = "deferred_webhook")]
    DeferredWebhook,
    #[serde(rename = "deferred_whatsapp")]
    DeferredWhatsapp,
    #[serde(rename = "delete_relationship")]
    DeleteRelationship,
    #[serde(rename = "delivered_action")]
    DeliveredAction,
    #[serde(rename = "delivered_email")]
    DeliveredEmail,
    #[serde(rename = "delivered_push")]
    DeliveredPush,
    #[serde(rename = "delivered_twilio")]
    DeliveredTwilio,
    #[serde(rename = "delivered_whatsapp")]
    DeliveredWhatsapp,
    #[serde(rename = "device_change")]
    DeviceChange,
    #[serde(rename = "drafted_action")]
    DraftedAction,
    #[serde(rename = "drafted_email")]
    DraftedEmail,
    #[serde(rename = "drafted_in_app")]
    DraftedInApp,
    #[serde(rename = "drafted_push")]
    DraftedPush,
    #[serde(rename = "drafted_slack")]
    DraftedSlack,
    #[serde(rename = "drafted_twilio")]
    DraftedTwilio,
    #[serde(rename = "drafted_webhook")]
    DraftedWebhook,
    #[serde(rename = "dropped_action")]
    DroppedAction,
    #[serde(rename = "dropped_email")]
    DroppedEmail,
    #[serde(rename = "dropped_push")]
    DroppedPush,
    #[serde(rename = "dropped_twilio")]
    DroppedTwilio,
    #[serde(rename = "dropped_webhook")]
    DroppedWebhook,
    #[serde(rename = "dropped_whatasapp")]
    DroppedWhatasapp,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "failed_action")]
    FailedAction,
    #[serde(rename = "failed_attribute_change")]
    FailedAttributeChange,
    #[serde(rename = "failed_batch_update")]
    FailedBatchUpdate,
    #[serde(rename = "failed_email")]
    FailedEmail,
    #[serde(rename = "failed_event")]
    FailedEvent,
    #[serde(rename = "failed_in_app")]
    FailedInApp,
    #[serde(rename = "failed_object_journeys")]
    FailedObjectJourneys,
    #[serde(rename = "failed_push")]
    FailedPush,
    #[serde(rename = "failed_query_collection")]
    FailedQueryCollection,
    #[serde(rename = "failed_slack")]
    FailedSlack,
    #[serde(rename = "failed_twilio")]
    FailedTwilio,
    #[serde(rename = "failed_webhook")]
    FailedWebhook,
    #[serde(rename = "failed_whatsapp")]
    FailedWhatsapp,
    #[serde(rename = "opened_action")]
    OpenedAction,
    #[serde(rename = "opened_email")]
    OpenedEmail,
    #[serde(rename = "opened_in_app")]
    OpenedInApp,
    #[serde(rename = "opened_push")]
    OpenedPush,
    #[serde(rename = "page")]
    Page,
    #[serde(rename = "profile_create")]
    ProfileCreate,
    #[serde(rename = "profile_delete")]
    ProfileDelete,
    #[serde(rename = "profile_merge")]
    ProfileMerge,
    #[serde(rename = "relationship_attribute_change")]
    RelationshipAttributeChange,
    #[serde(rename = "relationship_failed_attribute_change")]
    RelationshipFailedAttributeChange,
    #[serde(rename = "screen")]
    Screen,
    #[serde(rename = "sent_action")]
    SentAction,
    #[serde(rename = "sent_email")]
    SentEmail,
    #[serde(rename = "sent_in_app")]
    SentInApp,
    #[serde(rename = "sent_push")]
    SentPush,
    #[serde(rename = "sent_slack")]
    SentSlack,
    #[serde(rename = "sent_twilio")]
    SentTwilio,
    #[serde(rename = "sent_webhook")]
    SentWebhook,
    #[serde(rename = "sent_whatsapp")]
    SentWhatsapp,
    #[serde(rename = "skipped_update")]
    SkippedUpdate,
    #[serde(rename = "spammed_email")]
    SpammedEmail,
    #[serde(rename = "suppressed_twilio")]
    SuppressedTwilio,
    #[serde(rename = "suppressed_whatsapp")]
    SuppressedWhatsapp,
    #[serde(rename = "topic_unsubscribed_email")]
    TopicUnsubscribedEmail,
    #[serde(rename = "undeliverable_action")]
    UndeliverableAction,
    #[serde(rename = "undeliverable_email")]
    UndeliverableEmail,
    #[serde(rename = "undeliverable_in_app")]
    UndeliverableInApp,
    #[serde(rename = "undeliverable_push")]
    UndeliverablePush,
    #[serde(rename = "undeliverable_slack")]
    UndeliverableSlack,
    #[serde(rename = "undeliverable_twilio")]
    UndeliverableTwilio,
    #[serde(rename = "undeliverable_webhook")]
    UndeliverableWebhook,
    #[serde(rename = "undeliverable_whatsapp")]
    UndeliverableWhatsapp,
    #[serde(rename = "unsubscribed_action")]
    UnsubscribedAction,
    #[serde(rename = "unsubscribed_email")]
    UnsubscribedEmail,
    #[serde(rename = "viewed_content")]
    ViewedContent,
    #[serde(rename = "webhook_event")]
    WebhookEvent,
    #[serde(rename = "_o:<object_type_id>:add_relationship")]
    OColonLessThanObjectTypeIdGreaterThanColonAddRelationship,
    #[serde(rename = "_o:<object_type_id>:attribute_change")]
    OColonLessThanObjectTypeIdGreaterThanColonAttributeChange,
    #[serde(rename = "_o:<object_type_id>:create")]
    OColonLessThanObjectTypeIdGreaterThanColonCreate,
    #[serde(rename = "_o:<object_type_id>:delete")]
    OColonLessThanObjectTypeIdGreaterThanColonDelete,
    #[serde(rename = "_o:<object_type_id>:delete_relationship")]
    OColonLessThanObjectTypeIdGreaterThanColonDeleteRelationship,
    #[serde(rename = "_o:<object_type_id>:failed_attribute_change")]
    OColonLessThanObjectTypeIdGreaterThanColonFailedAttributeChange,
    #[serde(rename = "_r:<object_type_id>:attribute_change")]
    RColonLessThanObjectTypeIdGreaterThanColonAttributeChange,
    #[serde(rename = "_r:<object_type_id>:failed_attribute_change")]
    RColonLessThanObjectTypeIdGreaterThanColonFailedAttributeChange,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),

}

impl std::fmt::Display for ActivityTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AddRelationship => write!(f, "add_relationship"),
            Self::AnonMerge => write!(f, "anon_merge"),
            Self::AttemptedAction => write!(f, "attempted_action"),
            Self::AttemptedEmail => write!(f, "attempted_email"),
            Self::AttemptedInApp => write!(f, "attempted_in_app"),
            Self::AttemptedPush => write!(f, "attempted_push"),
            Self::AttemptedSlack => write!(f, "attempted_slack"),
            Self::AttemptedTwilio => write!(f, "attempted_twilio"),
            Self::AttemptedWebhook => write!(f, "attempted_webhook"),
            Self::AttemptedWhatsapp => write!(f, "attempted_whatsapp"),
            Self::AttributeChange => write!(f, "attribute_change"),
            Self::BouncedAction => write!(f, "bounced_action"),
            Self::BouncedEmail => write!(f, "bounced_email"),
            Self::BouncedPush => write!(f, "bounced_push"),
            Self::BouncedTwilio => write!(f, "bounced_twilio"),
            Self::BouncedWhatsapp => write!(f, "bounced_whatsapp"),
            Self::ClickedAction => write!(f, "clicked_action"),
            Self::ClickedContent => write!(f, "clicked_content"),
            Self::ClickedEmail => write!(f, "clicked_email"),
            Self::ClickedInApp => write!(f, "clicked_in_app"),
            Self::ClickedPush => write!(f, "clicked_push"),
            Self::ClickedTwilio => write!(f, "clicked_twilio"),
            Self::ClickedWebhook => write!(f, "clicked_webhook"),
            Self::ConvertedAction => write!(f, "converted_action"),
            Self::ConvertedContent => write!(f, "converted_content"),
            Self::ConvertedEmail => write!(f, "converted_email"),
            Self::ConvertedInApp => write!(f, "converted_in_app"),
            Self::ConvertedSlack => write!(f, "converted_slack"),
            Self::ConvertedTwilio => write!(f, "converted_twilio"),
            Self::ConvertedWebhook => write!(f, "converted_webhook"),
            Self::ConvertedWhatsapp => write!(f, "converted_whatsapp"),
            Self::DeferredAction => write!(f, "deferred_action"),
            Self::DeferredEmail => write!(f, "deferred_email"),
            Self::DeferredInApp => write!(f, "deferred_in_app"),
            Self::DeferredPush => write!(f, "deferred_push"),
            Self::DeferredSlack => write!(f, "deferred_slack"),
            Self::DeferredTwilio => write!(f, "deferred_twilio"),
            Self::DeferredWebhook => write!(f, "deferred_webhook"),
            Self::DeferredWhatsapp => write!(f, "deferred_whatsapp"),
            Self::DeleteRelationship => write!(f, "delete_relationship"),
            Self::DeliveredAction => write!(f, "delivered_action"),
            Self::DeliveredEmail => write!(f, "delivered_email"),
            Self::DeliveredPush => write!(f, "delivered_push"),
            Self::DeliveredTwilio => write!(f, "delivered_twilio"),
            Self::DeliveredWhatsapp => write!(f, "delivered_whatsapp"),
            Self::DeviceChange => write!(f, "device_change"),
            Self::DraftedAction => write!(f, "drafted_action"),
            Self::DraftedEmail => write!(f, "drafted_email"),
            Self::DraftedInApp => write!(f, "drafted_in_app"),
            Self::DraftedPush => write!(f, "drafted_push"),
            Self::DraftedSlack => write!(f, "drafted_slack"),
            Self::DraftedTwilio => write!(f, "drafted_twilio"),
            Self::DraftedWebhook => write!(f, "drafted_webhook"),
            Self::DroppedAction => write!(f, "dropped_action"),
            Self::DroppedEmail => write!(f, "dropped_email"),
            Self::DroppedPush => write!(f, "dropped_push"),
            Self::DroppedTwilio => write!(f, "dropped_twilio"),
            Self::DroppedWebhook => write!(f, "dropped_webhook"),
            Self::DroppedWhatasapp => write!(f, "dropped_whatasapp"),
            Self::Event => write!(f, "event"),
            Self::FailedAction => write!(f, "failed_action"),
            Self::FailedAttributeChange => write!(f, "failed_attribute_change"),
            Self::FailedBatchUpdate => write!(f, "failed_batch_update"),
            Self::FailedEmail => write!(f, "failed_email"),
            Self::FailedEvent => write!(f, "failed_event"),
            Self::FailedInApp => write!(f, "failed_in_app"),
            Self::FailedObjectJourneys => write!(f, "failed_object_journeys"),
            Self::FailedPush => write!(f, "failed_push"),
            Self::FailedQueryCollection => write!(f, "failed_query_collection"),
            Self::FailedSlack => write!(f, "failed_slack"),
            Self::FailedTwilio => write!(f, "failed_twilio"),
            Self::FailedWebhook => write!(f, "failed_webhook"),
            Self::FailedWhatsapp => write!(f, "failed_whatsapp"),
            Self::OpenedAction => write!(f, "opened_action"),
            Self::OpenedEmail => write!(f, "opened_email"),
            Self::OpenedInApp => write!(f, "opened_in_app"),
            Self::OpenedPush => write!(f, "opened_push"),
            Self::Page => write!(f, "page"),
            Self::ProfileCreate => write!(f, "profile_create"),
            Self::ProfileDelete => write!(f, "profile_delete"),
            Self::ProfileMerge => write!(f, "profile_merge"),
            Self::RelationshipAttributeChange => write!(f, "relationship_attribute_change"),
            Self::RelationshipFailedAttributeChange => write!(f, "relationship_failed_attribute_change"),
            Self::Screen => write!(f, "screen"),
            Self::SentAction => write!(f, "sent_action"),
            Self::SentEmail => write!(f, "sent_email"),
            Self::SentInApp => write!(f, "sent_in_app"),
            Self::SentPush => write!(f, "sent_push"),
            Self::SentSlack => write!(f, "sent_slack"),
            Self::SentTwilio => write!(f, "sent_twilio"),
            Self::SentWebhook => write!(f, "sent_webhook"),
            Self::SentWhatsapp => write!(f, "sent_whatsapp"),
            Self::SkippedUpdate => write!(f, "skipped_update"),
            Self::SpammedEmail => write!(f, "spammed_email"),
            Self::SuppressedTwilio => write!(f, "suppressed_twilio"),
            Self::SuppressedWhatsapp => write!(f, "suppressed_whatsapp"),
            Self::TopicUnsubscribedEmail => write!(f, "topic_unsubscribed_email"),
            Self::UndeliverableAction => write!(f, "undeliverable_action"),
            Self::UndeliverableEmail => write!(f, "undeliverable_email"),
            Self::UndeliverableInApp => write!(f, "undeliverable_in_app"),
            Self::UndeliverablePush => write!(f, "undeliverable_push"),
            Self::UndeliverableSlack => write!(f, "undeliverable_slack"),
            Self::UndeliverableTwilio => write!(f, "undeliverable_twilio"),
            Self::UndeliverableWebhook => write!(f, "undeliverable_webhook"),
            Self::UndeliverableWhatsapp => write!(f, "undeliverable_whatsapp"),
            Self::UnsubscribedAction => write!(f, "unsubscribed_action"),
            Self::UnsubscribedEmail => write!(f, "unsubscribed_email"),
            Self::ViewedContent => write!(f, "viewed_content"),
            Self::WebhookEvent => write!(f, "webhook_event"),
            Self::OColonLessThanObjectTypeIdGreaterThanColonAddRelationship => write!(f, "_o:<object_type_id>:add_relationship"),
            Self::OColonLessThanObjectTypeIdGreaterThanColonAttributeChange => write!(f, "_o:<object_type_id>:attribute_change"),
            Self::OColonLessThanObjectTypeIdGreaterThanColonCreate => write!(f, "_o:<object_type_id>:create"),
            Self::OColonLessThanObjectTypeIdGreaterThanColonDelete => write!(f, "_o:<object_type_id>:delete"),
            Self::OColonLessThanObjectTypeIdGreaterThanColonDeleteRelationship => write!(f, "_o:<object_type_id>:delete_relationship"),
            Self::OColonLessThanObjectTypeIdGreaterThanColonFailedAttributeChange => write!(f, "_o:<object_type_id>:failed_attribute_change"),
            Self::RColonLessThanObjectTypeIdGreaterThanColonAttributeChange => write!(f, "_r:<object_type_id>:attribute_change"),
            Self::RColonLessThanObjectTypeIdGreaterThanColonFailedAttributeChange => write!(f, "_r:<object_type_id>:failed_attribute_change"),
            Self::UnknownValue(s) => write!(f, "{s}"),
        }
    }
}

impl Default for ActivityTypes {
    fn default() -> ActivityTypes {
        Self::AddRelationship
    }
}

