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

/// PeriodMessageMetrics : Metrics grouped by the requested resolution. Each property is an array where each entry represents one period, such as one day.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeriodMessageMetrics {
    /// The number of `attempted` messages.
    #[serde(rename = "attempted", skip_serializing_if = "Option::is_none")]
    pub attempted: Option<Vec<i64>>,
    /// The number of `bounced` messages.
    #[serde(rename = "bounced", skip_serializing_if = "Option::is_none")]
    pub bounced: Option<Vec<i64>>,
    /// The number of `clicked` messages.
    #[serde(rename = "clicked", skip_serializing_if = "Option::is_none")]
    pub clicked: Option<Vec<i64>>,
    /// The number of `converted` messages.
    #[serde(rename = "converted", skip_serializing_if = "Option::is_none")]
    pub converted: Option<Vec<i64>>,
    /// The number of `created` messages.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<Vec<i64>>,
    /// The number of `deferred` messages.
    #[serde(rename = "deferred", skip_serializing_if = "Option::is_none")]
    pub deferred: Option<Vec<i64>>,
    /// The number of `delivered` messages.
    #[serde(rename = "delivered", skip_serializing_if = "Option::is_none")]
    pub delivered: Option<Vec<i64>>,
    /// The number of `drafted` messages.
    #[serde(rename = "drafted", skip_serializing_if = "Option::is_none")]
    pub drafted: Option<Vec<i64>>,
    /// The number of `failed` messages.
    #[serde(rename = "failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<i64>>,
    /// The number of `clicked` emails excluding machine clicks. This metric is reliable starting April 20, 2025. [Learn more](/messaging/metrics/analytics/#delivery-metrics).
    #[serde(rename = "human_clicked", skip_serializing_if = "Option::is_none")]
    pub human_clicked: Option<Vec<i64>>,
    /// The number of `opened` emails excluding machine opens. This metric is reliable starting March 20, 2025. [Learn more](/messaging/metrics/analytics/#delivery-metrics).
    #[serde(rename = "human_opened", skip_serializing_if = "Option::is_none")]
    pub human_opened: Option<Vec<i64>>,
    /// The number of emails delivered with link tracking inactive for any reason. We subtract these from the click-rate denominator.
    #[serde(rename = "link_untracked", skip_serializing_if = "Option::is_none")]
    pub link_untracked: Option<Vec<i64>>,
    /// The number of `clicked` emails attributed to machines—security scanners, bot user agents, known proxy services, and repetitive click patterns. This metric is reliable starting April 20, 2025. [Learn more](/messaging/metrics/analytics/#delivery-metrics).
    #[serde(rename = "machine_clicked", skip_serializing_if = "Option::is_none")]
    pub machine_clicked: Option<Vec<i64>>,
    /// The number of emails delivered with open tracking inactive for any reason—including the consent opt-outs that `untracked` counts and messages sent with open tracking disabled. We subtract these from the open-rate denominator.
    #[serde(rename = "open_untracked", skip_serializing_if = "Option::is_none")]
    pub open_untracked: Option<Vec<i64>>,
    /// The number of `opened` messages.
    #[serde(rename = "opened", skip_serializing_if = "Option::is_none")]
    pub opened: Option<Vec<i64>>,
    /// The number of `opened` emails attributed to machines. This metric is reliable starting March 20, 2025.
    #[serde(rename = "prefetch_opened", skip_serializing_if = "Option::is_none")]
    pub prefetch_opened: Option<Vec<i64>>,
    /// The number of SMS/MMS messages a person replied to with a keyword. [Learn more about inbound messages](/messaging/channels/sms/inbound/get-started/).
    #[serde(rename = "replied", skip_serializing_if = "Option::is_none")]
    pub replied: Option<Vec<i64>>,
    /// The number of sent messages.
    #[serde(rename = "sent", skip_serializing_if = "Option::is_none")]
    pub sent: Option<Vec<i64>>,
    /// The number of spam complaints.
    #[serde(rename = "spammed", skip_serializing_if = "Option::is_none")]
    pub spammed: Option<Vec<i64>>,
    /// The number of `suppressed` messages.
    #[serde(rename = "suppressed", skip_serializing_if = "Option::is_none")]
    pub suppressed: Option<Vec<i64>>,
    /// The number of topic unsubscribes in a given period.
    #[serde(rename = "topic_unsubscribed", skip_serializing_if = "Option::is_none")]
    pub topic_unsubscribed: Option<Vec<i64>>,
    /// The number of people who denied consent to email open tracking in the period.
    #[serde(rename = "tracking_consent_denied", skip_serializing_if = "Option::is_none")]
    pub tracking_consent_denied: Option<Vec<i64>>,
    /// The number of people who granted consent to email open tracking in the period.
    #[serde(rename = "tracking_consent_granted", skip_serializing_if = "Option::is_none")]
    pub tracking_consent_granted: Option<Vec<i64>>,
    /// The number of `undeliverable` messages.
    #[serde(rename = "undeliverable", skip_serializing_if = "Option::is_none")]
    pub undeliverable: Option<Vec<i64>>,
    /// The number of unsubscribes attributed to the automation or message.
    #[serde(rename = "unsubscribed", skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<Vec<i64>>,
    /// The number of `sent` emails whose open pixel we withheld because the person opted out of open tracking. We exclude these from the open-rate denominator; click tracking is unaffected.
    #[serde(rename = "untracked", skip_serializing_if = "Option::is_none")]
    pub untracked: Option<Vec<i64>>,
}

impl PeriodMessageMetrics {
    /// Metrics grouped by the requested resolution. Each property is an array where each entry represents one period, such as one day.
    pub fn new() -> PeriodMessageMetrics {
        PeriodMessageMetrics {
            attempted: None,
            bounced: None,
            clicked: None,
            converted: None,
            created: None,
            deferred: None,
            delivered: None,
            drafted: None,
            failed: None,
            human_clicked: None,
            human_opened: None,
            link_untracked: None,
            machine_clicked: None,
            open_untracked: None,
            opened: None,
            prefetch_opened: None,
            replied: None,
            sent: None,
            spammed: None,
            suppressed: None,
            topic_unsubscribed: None,
            tracking_consent_denied: None,
            tracking_consent_granted: None,
            undeliverable: None,
            unsubscribed: None,
            untracked: None,
        }
    }
}

