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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportingWebhook {
    /// The webhook URL.
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    /// Specifies the types of events you want to report to your webhook. See our [reporting webhooks reference](/integrations/api/webhooks/) for more information about event types and the information they return.
    #[serde(rename = "events")]
    pub events: Vec<Events>,
    /// The name of your webhook.
    #[serde(rename = "name")]
    pub name: String,
    /// Set to `true` to quit sending events to the webhook URL. Set to `false` to enable the webhook.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Set to `false` to send unique open and click events to the webhook. Set to `true` to send all events.
    #[serde(rename = "full_resolution", skip_serializing_if = "Option::is_none")]
    pub full_resolution: Option<bool>,
    /// The identifier for the webhook.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The type of webhook—`webhook` for a standard reporting webhook or `js` for a JavaScript webhook.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Set to `true` to include the message `body` in `_sent` events.
    #[serde(rename = "with_content", skip_serializing_if = "Option::is_none")]
    pub with_content: Option<bool>,
}

impl ReportingWebhook {
    pub fn new(endpoint: String, events: Vec<Events>, name: String) -> ReportingWebhook {
        ReportingWebhook {
            endpoint,
            events,
            name,
            disabled: None,
            full_resolution: None,
            id: None,
            r#type: None,
            with_content: None,
        }
    }
}
/// Specifies the types of events you want to report to your webhook. See our [reporting webhooks reference](/integrations/api/webhooks/) for more information about event types and the information they return.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Events {
    #[serde(rename = "customer_subscribed")]
    CustomerSubscribed,
    #[serde(rename = "customer_unsubscribed")]
    CustomerUnsubscribed,
    #[serde(rename = "customer_subscription_preferences_changed")]
    CustomerSubscriptionPreferencesChanged,
    #[serde(rename = "email_drafted")]
    EmailDrafted,
    #[serde(rename = "email_attempted")]
    EmailAttempted,
    #[serde(rename = "email_sent")]
    EmailSent,
    #[serde(rename = "email_delivered")]
    EmailDelivered,
    #[serde(rename = "email_opened")]
    EmailOpened,
    #[serde(rename = "email_clicked")]
    EmailClicked,
    #[serde(rename = "email_converted")]
    EmailConverted,
    #[serde(rename = "email_bounced")]
    EmailBounced,
    #[serde(rename = "email_dropped")]
    EmailDropped,
    #[serde(rename = "email_deferred")]
    EmailDeferred,
    #[serde(rename = "email_spammed")]
    EmailSpammed,
    #[serde(rename = "email_failed")]
    EmailFailed,
    #[serde(rename = "email_unsubscribed")]
    EmailUnsubscribed,
    #[serde(rename = "email_undeliverable")]
    EmailUndeliverable,
    #[serde(rename = "in_app_drafted")]
    InAppDrafted,
    #[serde(rename = "in_app_attempted")]
    InAppAttempted,
    #[serde(rename = "in_app_sent")]
    InAppSent,
    #[serde(rename = "in_app_opened")]
    InAppOpened,
    #[serde(rename = "in_app_clicked")]
    InAppClicked,
    #[serde(rename = "in_app_converted")]
    InAppConverted,
    #[serde(rename = "in_app_failed")]
    InAppFailed,
    #[serde(rename = "in_app_undeliverable")]
    InAppUndeliverable,
    #[serde(rename = "push_drafted")]
    PushDrafted,
    #[serde(rename = "push_attempted")]
    PushAttempted,
    #[serde(rename = "push_sent")]
    PushSent,
    #[serde(rename = "push_delivered")]
    PushDelivered,
    #[serde(rename = "push_opened")]
    PushOpened,
    #[serde(rename = "push_clicked")]
    PushClicked,
    #[serde(rename = "push_converted")]
    PushConverted,
    #[serde(rename = "push_bounced")]
    PushBounced,
    #[serde(rename = "push_dropped")]
    PushDropped,
    #[serde(rename = "push_failed")]
    PushFailed,
    #[serde(rename = "push_undeliverable")]
    PushUndeliverable,
    #[serde(rename = "slack_drafted")]
    SlackDrafted,
    #[serde(rename = "slack_attempted")]
    SlackAttempted,
    #[serde(rename = "slack_sent")]
    SlackSent,
    #[serde(rename = "slack_clicked")]
    SlackClicked,
    #[serde(rename = "slack_converted")]
    SlackConverted,
    #[serde(rename = "slack_failed")]
    SlackFailed,
    #[serde(rename = "sms_drafted")]
    SmsDrafted,
    #[serde(rename = "sms_attempted")]
    SmsAttempted,
    #[serde(rename = "sms_sent")]
    SmsSent,
    #[serde(rename = "sms_delivered")]
    SmsDelivered,
    #[serde(rename = "sms_clicked")]
    SmsClicked,
    #[serde(rename = "sms_converted")]
    SmsConverted,
    #[serde(rename = "sms_bounced")]
    SmsBounced,
    #[serde(rename = "sms_dropped")]
    SmsDropped,
    #[serde(rename = "sms_failed")]
    SmsFailed,
    #[serde(rename = "sms_undeliverable")]
    SmsUndeliverable,
    #[serde(rename = "sms_replied")]
    SmsReplied,
    #[serde(rename = "whatsapp_drafted")]
    WhatsappDrafted,
    #[serde(rename = "whatsapp_attempted")]
    WhatsappAttempted,
    #[serde(rename = "whatsapp_sent")]
    WhatsappSent,
    #[serde(rename = "whatsapp_delivered")]
    WhatsappDelivered,
    #[serde(rename = "whatsapp_opened")]
    WhatsappOpened,
    #[serde(rename = "whatsapp_clicked")]
    WhatsappClicked,
    #[serde(rename = "whatsapp_converted")]
    WhatsappConverted,
    #[serde(rename = "whatsapp_bounced")]
    WhatsappBounced,
    #[serde(rename = "whatsapp_dropped")]
    WhatsappDropped,
    #[serde(rename = "whatsapp_failed")]
    WhatsappFailed,
    #[serde(rename = "whatsapp_undeliverable")]
    WhatsappUndeliverable,
    #[serde(rename = "whatsapp_replied")]
    WhatsappReplied,
    #[serde(rename = "webhook_drafted")]
    WebhookDrafted,
    #[serde(rename = "webhook_attempted")]
    WebhookAttempted,
    #[serde(rename = "webhook_sent")]
    WebhookSent,
    #[serde(rename = "webhook_clicked")]
    WebhookClicked,
    #[serde(rename = "webhook_converted")]
    WebhookConverted,
    #[serde(rename = "webhook_failed")]
    WebhookFailed,
    #[serde(rename = "webhook_undeliverable")]
    WebhookUndeliverable,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for Events {
    fn default() -> Events {
        Self::CustomerSubscribed
    }
}
/// The type of webhook—`webhook` for a standard reporting webhook or `js` for a JavaScript webhook.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "webhook")]
    Webhook,
    #[serde(rename = "js")]
    Js,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for Type {
    fn default() -> Type {
        Self::Webhook
    }
}

