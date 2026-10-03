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

/// PushEvents : Describes the push events reported from Customer.io to a webhook.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PushEvents {
    /// Reports when a push notification could not be sent to the delivery provider will retry. Set to true to report this event type.
    #[serde(rename = "push_attempted", skip_serializing_if = "Option::is_none")]
    pub push_attempted: Option<bool>,
    /// Reports when the delivery provider is unable to deliver a message. Set to true to report this event type.
    #[serde(rename = "push_bounced", skip_serializing_if = "Option::is_none")]
    pub push_bounced: Option<bool>,
    /// Reports when a person clicks a tracked link in a message. Set to true to report this event type.
    #[serde(rename = "push_clicked", skip_serializing_if = "Option::is_none")]
    pub push_clicked: Option<bool>,
    /// Reports a conversion. Set to true to report this event type.
    #[serde(rename = "push_converted", skip_serializing_if = "Option::is_none")]
    pub push_converted: Option<bool>,
    /// An app reports that the recipient's device received a message. Set to true to report this event type.
    #[serde(rename = "push_delivered", skip_serializing_if = "Option::is_none")]
    pub push_delivered: Option<bool>,
    /// Reports when a message draft is created. Set to true to report this event type.
    #[serde(rename = "push_drafted", skip_serializing_if = "Option::is_none")]
    pub push_drafted: Option<bool>,
    /// Reports when a message isn't sent because the recipient is suppressed. Set to true to report this event type.
    #[serde(rename = "push_dropped", skip_serializing_if = "Option::is_none")]
    pub push_dropped: Option<bool>,
    /// Reports when a message couldn't be sent to the delivery provider. Set to true to report this event type.
    #[serde(rename = "push_failed", skip_serializing_if = "Option::is_none")]
    pub push_failed: Option<bool>,
    /// The app on a recipient's device reports that the recipient opened the message. Set to true to report this event type.
    #[serde(rename = "push_opened", skip_serializing_if = "Option::is_none")]
    pub push_opened: Option<bool>,
    /// Reports when a message is sent from Customer.io to the delivery provider. Set to true to report this event type.
    #[serde(rename = "push_sent", skip_serializing_if = "Option::is_none")]
    pub push_sent: Option<bool>,
}

impl PushEvents {
    /// Describes the push events reported from Customer.io to a webhook.
    pub fn new() -> PushEvents {
        PushEvents {
            push_attempted: None,
            push_bounced: None,
            push_clicked: None,
            push_converted: None,
            push_delivered: None,
            push_drafted: None,
            push_dropped: None,
            push_failed: None,
            push_opened: None,
            push_sent: None,
        }
    }
}

