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
pub struct Sendpush {
    #[serde(rename = "identifiers")]
    pub identifiers: Box<models::TransactionalSharedObjectIdentifiers>,
    #[serde(rename = "transactional_message_id")]
    pub transactional_message_id: Box<models::SendpushTransactionalMessageId>,
    /// If `true` and your `transactional_message_id` doesn't match a record, Customer.io creates an empty record using that value as the *Trigger Name*. The ID must be a string. If the name already belongs to another channel, the request fails with `400`, and numeric IDs ignore this setting. See [details](/journeys/transactional-email/#auto-create-transactional-message-records).
    #[serde(rename = "auto_create", skip_serializing_if = "Option::is_none")]
    pub auto_create: Option<bool>,
    /// Optional key/value pairs you want to attach to the push payload. Firebase only supports string values. This overrides the Custom Data from your transactional template.
    #[serde(rename = "custom_data", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<serde_json::Value>,
    #[serde(rename = "custom_device", skip_serializing_if = "Option::is_none")]
    pub custom_device: Option<Box<models::TransactionalSharedPushObjectCustomDevice>>,
    #[serde(rename = "custom_payload", skip_serializing_if = "Option::is_none")]
    pub custom_payload: Option<Box<models::TransactionalSharedPushObjectCustomPayload>>,
    /// If true, the message body is not retained in delivery history. Setting this value overrides the value set in the settings of your `transactional_message_id`.
    #[serde(rename = "disable_message_retention", skip_serializing_if = "Option::is_none")]
    pub disable_message_retention: Option<bool>,
    /// An image URL to show in the push. This overrides Image from the transactional template (referenced by `transactional_message_id`).
    #[serde(rename = "image_url", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Overrides language preferences for the person you want to send your transactional message to. Use one of our [supported two- or four-letter language codes](/journeys/channels/localization/getting-started/#supported-languages).
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// A deep link to open when the push is tapped. This overrides Link from the transactional template (referenced by `transactional_message_id`).
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// The message body for your notification. This overrides the notification body of the transactional template (referenced by `transactional_message_id`).
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// An object containing the key-value pairs referenced using liquid in your message.
    #[serde(rename = "message_data", skip_serializing_if = "Option::is_none")]
    pub message_data: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// If true, your transactional message is held as a draft in Customer.io and not sent directly to your audience. You must go to the Deliveries and Drafts page to send your message.
    #[serde(rename = "queue_draft", skip_serializing_if = "Option::is_none")]
    pub queue_draft: Option<bool>,
    /// A unix timestamp (seconds since epoch) determining when the message will be sent. The timestamp can be up to 90 days in the future. If this value is in the past, your message is sent immediately.
    #[serde(rename = "send_at", skip_serializing_if = "Option::is_none")]
    pub send_at: Option<i64>,
    /// If false, your message is not sent to unsubscribed recipients. Setting this value overrides the value set in the settings of your `transactional_message_id`.
    #[serde(rename = "send_to_unsubscribed", skip_serializing_if = "Option::is_none")]
    pub send_to_unsubscribed: Option<bool>,
    /// **For iOS Only**: your notification can alert users with the device's default notification sound or play no sound at all. 
    #[serde(rename = "sound", skip_serializing_if = "Option::is_none")]
    pub sound: Option<Sound>,
    /// The title for your notification. This overrides the title of the transactional template (referenced by `transactional_message_id`).
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The devices you want to send this push to—`all`, `last_used`, or a custom device token from the identified profile. Defaults to `all` and overrides the `To` value from your transactional template.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<To>,
}

impl Sendpush {
    pub fn new(identifiers: models::TransactionalSharedObjectIdentifiers, transactional_message_id: models::SendpushTransactionalMessageId) -> Sendpush {
        Sendpush {
            identifiers: Box::new(identifiers),
            transactional_message_id: Box::new(transactional_message_id),
            auto_create: None,
            custom_data: None,
            custom_device: None,
            custom_payload: None,
            disable_message_retention: None,
            image_url: None,
            language: None,
            link: None,
            message: None,
            message_data: None,
            queue_draft: None,
            send_at: None,
            send_to_unsubscribed: None,
            sound: None,
            title: None,
            to: None,
        }
    }
}
/// **For iOS Only**: your notification can alert users with the device's default notification sound or play no sound at all. 
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Sound {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "none")]
    None,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for Sound {
    fn default() -> Sound {
        Self::Default
    }
}
/// The devices you want to send this push to—`all`, `last_used`, or a custom device token from the identified profile. Defaults to `all` and overrides the `To` value from your transactional template.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum To {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "last_used")]
    LastUsed,
    #[serde(rename = "$device_token")]
    DollarDeviceToken,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for To {
    fn default() -> To {
        Self::All
    }
}

