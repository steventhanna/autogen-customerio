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
pub struct EmailMessage3 {
    /// The blind-copy address(es) for this action.
    #[serde(rename = "bcc", skip_serializing_if = "Option::is_none")]
    pub bcc: Option<String>,
    /// The body of the action. You cannot modify the body if you created it with our drag-and-drop editor.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// AMP-enabled content for your email. If a recipient's email client doesn't support AMP, they receive your `body` content instead. Make sure you're [set up to send AMP](/journeys/channels/email/layouts/amp-for-email/) first.
    #[serde(rename = "body_amp", skip_serializing_if = "Option::is_none")]
    pub body_amp: Option<String>,
    /// The identifier for a broadcast.
    #[serde(rename = "broadcast_id", skip_serializing_if = "Option::is_none")]
    pub broadcast_id: Option<crate::ids::BroadcastId>,
    /// The identifier for an automation.
    #[serde(rename = "campaign_id", skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<crate::ids::CampaignId>,
    /// The carbon-copy address(es) for this action.
    #[serde(rename = "cc", skip_serializing_if = "Option::is_none")]
    pub cc: Option<String>,
    /// The date time when the referenced ID was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// An identifier in the format `id:timestamp` where the id is for the object you're working with (Automations, Deliveries, Exports, Identities, One-time sends, Segments, and Templates), and the timestamp is the last time the object was updated.
    #[serde(rename = "deduplicate_id", skip_serializing_if = "Option::is_none")]
    pub deduplicate_id: Option<String>,
    /// If true, rather than sending true copies to BCC addresses, Customer.io sends a copy of the message with the subject line containing the recipient address(es). 
    #[serde(rename = "fake_bcc", skip_serializing_if = "Option::is_none")]
    pub fake_bcc: Option<bool>,
    /// The address that the message is from, relevant if the action `type` is `email`.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// The identifier of the `from` address, commonly known as the \"sender\". Use [List sender identities](/integrations/api/app/tag/sender-identities/listSenders/) to find valid IDs.
    #[serde(rename = "from_id", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<crate::ids::SenderIdentityId>,
    /// A JSON string containing header objects with `name` and `value`. Names and values must be strings, with no non-ASCII characters or spaces. You can't overwrite reserved headers.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<String>,
    /// The identifier for an action.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The language variant for your message. If you don't use our [localization feature](/journeys/channels/localization/getting-started), or this is the default message, this value is an empty string.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The layout used for the action, if it exists.
    #[serde(rename = "layout", skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    /// Returned when the action is a language variant of a multi-language message. This id is the same for each variant of a message, so you can use this to retrieve metrics for a message and all its variants.
    #[serde(rename = "multi_language_branch_action_id", skip_serializing_if = "Option::is_none")]
    pub multi_language_branch_action_id: Option<crate::ids::ActionId>,
    /// The name of the action, if it exists.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ID of the parent action, if the action occurred within an automation and has a parent (like a randomized split, etc).
    #[serde(rename = "parent_action_id", skip_serializing_if = "Option::is_none")]
    pub parent_action_id: Option<crate::ids::ActionId>,
    /// [Also known as \"preview text\"](/journeys/channels/email/headers/custom-preheader-text/), this is the small block of text shown in an email inbox next to or underneath the subject line. 
    #[serde(rename = "preheader_text", skip_serializing_if = "Option::is_none")]
    pub preheader_text: Option<String>,
    #[serde(rename = "preprocessor", skip_serializing_if = "Option::is_none")]
    pub preprocessor: Option<models::Preprocessor>,
    /// The recipient address for an action.
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// The address that receives replies for the message, if applicable.
    #[serde(rename = "reply_to", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// The identifier for the `reply_to` address, if applicable. Use [List sender identities](/integrations/api/app/tag/sender-identities/listSenders/) to find valid IDs.
    #[serde(rename = "reply_to_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reply_to_id: Option<Option<crate::ids::SenderIdentityId>>,
    #[serde(rename = "sending_state", skip_serializing_if = "Option::is_none")]
    pub sending_state: Option<models::SendingState>,
    /// The subject line for an `email` action.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The type of action.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// The date time when the referenced ID was last updated.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
}

impl EmailMessage3 {
    pub fn new() -> EmailMessage3 {
        EmailMessage3 {
            bcc: None,
            body: None,
            body_amp: None,
            broadcast_id: None,
            campaign_id: None,
            cc: None,
            created: None,
            deduplicate_id: None,
            fake_bcc: None,
            from: None,
            from_id: None,
            headers: None,
            id: None,
            language: None,
            layout: None,
            multi_language_branch_action_id: None,
            name: None,
            parent_action_id: None,
            preheader_text: None,
            preprocessor: None,
            recipient: None,
            reply_to: None,
            reply_to_id: None,
            sending_state: None,
            subject: None,
            r#type: None,
            updated: None,
        }
    }
}
/// The type of action.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "in_app")]
    InApp,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "twilio")]
    Twilio,
    #[serde(rename = "slack")]
    Slack,
    #[serde(rename = "whatsapp")]
    Whatsapp,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for Type {
    fn default() -> Type {
        Self::Email
    }
}

