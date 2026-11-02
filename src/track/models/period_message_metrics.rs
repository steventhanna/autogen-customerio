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

