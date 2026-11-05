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
pub struct Sendemail {
    /// The recipients you want to send to, separated by commas. You can include up to 15 total recipients across `to`, `cc`, and `bcc`, with optional display names in quotes.
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Box<models::Attachments>>,
    /// If `true` and your `transactional_message_id` doesn't match a record, Customer.io creates an empty record using that value as the *Trigger Name*. The ID must be a string. If the name already belongs to another channel, the request fails with `400`, and numeric IDs ignore this setting. See [details](/journeys/transactional-email/#auto-create-transactional-message-records).
    #[serde(rename = "auto_create", skip_serializing_if = "Option::is_none")]
    pub auto_create: Option<bool>,
    /// Blind copy message recipients. Supports multiple addresses separated by commas. Your request can contain up to 15 total recipients between the `to`, `cc`, and `bcc` keys.
    #[serde(rename = "bcc", skip_serializing_if = "Option::is_none")]
    pub bcc: Option<String>,
    /// The HTML body of your message. If you provide a `transactional_message_id`, this overrides the template's body. It's also the fallback when you send AMP email (`body_amp`) to a client that doesn't support AMP.
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// AMP-enabled content for your email. If a recipient's email client doesn't support AMP, they receive your `body` content instead. Make sure you're [set up to send AMP](/journeys/channels/email/layouts/amp-for-email/) first.
    #[serde(rename = "body_amp", skip_serializing_if = "Option::is_none")]
    pub body_amp: Option<String>,
    /// The plaintext body of your message. If you provide `transactional_message_id`, this overrides the template's plaintext body.
    #[serde(rename = "body_plain", skip_serializing_if = "Option::is_none")]
    pub body_plain: Option<String>,
    /// Carbon copy message recipients, separated by commas. Unlike BCC recipients, CC recipients are visible to everyone who receives the message. Their opens, clicks, and bounces count toward your message metrics. CC recipients count toward the limit of 15 total recipients across the `to`, `cc`, and `bcc` keys.
    #[serde(rename = "cc", skip_serializing_if = "Option::is_none")]
    pub cc: Option<String>,
    /// Set to `true` to disable CSS preprocessing. This setting overrides the CSS preprocessing setting on the `transactional_message_id` as set in the user interface. Transactional emails have CSS preprocessing enabled by default.
    #[serde(rename = "disable_css_preprocessing", skip_serializing_if = "Option::is_none")]
    pub disable_css_preprocessing: Option<bool>,
    /// If true, the message body is not retained in delivery history. Setting this value overrides the value set in the settings of your `transactional_message_id`.
    #[serde(rename = "disable_message_retention", skip_serializing_if = "Option::is_none")]
    pub disable_message_retention: Option<bool>,
    /// If true, rather than sending true copies to BCC addresses, Customer.io sends a copy of the message with the subject line containing the recipient address(es). 
    #[serde(rename = "fake_bcc", skip_serializing_if = "Option::is_none")]
    pub fake_bcc: Option<bool>,
    /// The address your email is from. It must be a [verified sender](/journeys/channels/email/deliverability/authentication/). Quote any display name, like `\\\"Person\\\" <person@example.com>`. This overrides the template's sender; omit it to use the template or your workspace default.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// A JSON string containing header objects with `name` and `value`. Names and values must be strings, with no non-ASCII characters or spaces. You can't overwrite reserved headers.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<String>,
    #[serde(rename = "identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Box<models::TransactionalSharedObjectIdentifiers>>,
    /// Overrides language preferences for the person you want to send your transactional message to. Use one of our [supported two- or four-letter language codes](/journeys/channels/localization/getting-started/#supported-languages).
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// An object containing the key-value pairs referenced using liquid in your message.
    #[serde(rename = "message_data", skip_serializing_if = "Option::is_none")]
    pub message_data: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Also known as \"preview text\", this is the block block of text that users see next to, or underneath, the subject line in their inbox.
    #[serde(rename = "preheader", skip_serializing_if = "Option::is_none")]
    pub preheader: Option<String>,
    /// If true, your transactional message is held as a draft in Customer.io and not sent directly to your audience. You must go to the Deliveries and Drafts page to send your message.
    #[serde(rename = "queue_draft", skip_serializing_if = "Option::is_none")]
    pub queue_draft: Option<bool>,
    /// The address that recipients can reply to, if different from the `from` address.
    #[serde(rename = "reply_to", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// A unix timestamp (seconds since epoch) determining when the message will be sent. The timestamp can be up to 90 days in the future. If this value is in the past, your message is sent immediately.
    #[serde(rename = "send_at", skip_serializing_if = "Option::is_none")]
    pub send_at: Option<i64>,
    /// If false, your message is not sent to unsubscribed recipients. Setting this value overrides the value set in the settings of your `transactional_message_id`.
    #[serde(rename = "send_to_unsubscribed", skip_serializing_if = "Option::is_none")]
    pub send_to_unsubscribed: Option<bool>,
    /// The subject line for your message. If you provide `transactional_message_id`, this overrides the template's subject.
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// If true, Customer.io tracks opens and link clicks in your message.
    #[serde(rename = "tracked", skip_serializing_if = "Option::is_none")]
    pub tracked: Option<bool>,
    #[serde(rename = "transactional_message_id", skip_serializing_if = "Option::is_none")]
    pub transactional_message_id: Option<Box<models::SendpushTransactionalMessageId>>,
}

impl Sendemail {
    pub fn new(to: String) -> Sendemail {
        Sendemail {
            to,
            attachments: None,
            auto_create: None,
            bcc: None,
            body: None,
            body_amp: None,
            body_plain: None,
            cc: None,
            disable_css_preprocessing: None,
            disable_message_retention: None,
            fake_bcc: None,
            from: None,
            headers: None,
            identifiers: None,
            language: None,
            message_data: None,
            preheader: None,
            queue_draft: None,
            reply_to: None,
            send_at: None,
            send_to_unsubscribed: None,
            subject: None,
            tracked: None,
            transactional_message_id: None,
        }
    }
}

