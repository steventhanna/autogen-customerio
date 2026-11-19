/*
 * Pipelines API
 *
 * # Overview In general, you'll consume this API through one of our source libraries—our JavaScript client library or any of our server packages. But you can also integrate directly with our REST API if you don't want to install one of our libraries or you want to support a source that we don't have a native integration with.  # Server addresses: US and EU Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://cdp.customer.io | | EU | https://cdp-eu.customer.io |  If you're in our EU region, you'll need to specify the EU URL when you initialize our server-side libraries. If you use our JavaScript client library, we'll set your region and route data/calls automatically.  # Authentication & rate limits  Use a **private key** (`sk_`) as a bearer token: `Authorization: Bearer sk_us_...`. To get one, see **Get an API key** below. In a browser or mobile app, use a public key (`pk_`). [Learn more about public and private keys](/accounts/settings/managing-credentials/#public-and-private-keys).  **Legacy credentials:** If you already have a **Customer.io API** integration with a write key, you can keep using it with Basic authentication: the key is the *Username*, and the *Password* is blank. Each operation on this page lists both kinds of credentials.  # Get an API key  [Set up the **Customer.io API** integration](/integrations/data-in/connections/http-api/) in your workspace. Customer.io creates a private key for it and shows the key only once, so copy it before you leave the setup page. To add another key to the integration later, go to its **Settings** tab, and in the **API keys** section, click **Create API key**.  ## Rate and payload limits  A request is limited to 32KB. A batch request is limited to 500KB total and 32KB per call in the request. If a request exceeds these limits, Customer.io doesn't process it. By default, Customer.io still returns a `200` response. In [strict mode](#strict-mode), Customer.io returns a `400` instead.  The Pipelines API has no fixed rate limit. Customer.io accepts your requests as fast as you send them and doesn't reject or drop them because of volume. Our guideline for both active integrations and historical backfills is about 3000 requests per 3 seconds, an average of 1000 requests per second. The 3-second window leaves room for short bursts.  Traffic above this level usually comes from high-volume integrations, which our Premium and Enterprise plans support. If your traffic regularly goes above this, we may contact you to learn more about your integration and talk about optimizing it, or about a plan that fits your volume.  <div class=\"fly-panel fly-light regionUS\"> <div class=\"fly-panel-body us-server\"> <p class=\"text--bold\">Warning</p> <p>Rate limits are subject to change. We may adjust these thresholds to ensure stable performance for all customers.</p> </div></div>  # Strict mode  The Pipelines API runs in *lax mode* by default. It returns a `200` response for almost every request, and logs issues after the fact in Customer.io. Lax mode keeps a bad request from breaking your integration, but it also means that a `200` response doesn't confirm that Customer.io processed your data.  Turn on *strict mode* to validate each request and return an error when it fails. This can help you build and test an integration, so you see each problem as soon as it happens. A strict mode error response contains an `errors` array. Each error has a `reason`, the `field` that failed, and a `message` that describes the problem. Errors from a `/batch` request also include an `index`, which is the position of the failing call in the `batch` array.  To turn on *strict mode*, send the `X-Strict-Mode` header with a value of `1`. If you leave the header out or send any other value, the request runs in lax mode.  | Problem with the request | Strict mode (`X-Strict-Mode: 1`) | Lax mode (default) | | :-- | :-- | :-- | | Missing or invalid API key | `401` | `200`, Customer.io drops the request | | Disabled API key | `401` | `404` | | A `Content-Encoding` other than `gzip` or `deflate` | `400` | `200`, Customer.io drops the request | | A compressed body that we can't decompress | `500` | `200`, Customer.io drops the request | | Over the [size limits](#rate-and-payload-limits) | `400` | `200`, Customer.io drops the request | | Body that isn't valid JSON | `400` | `200`, Customer.io drops the request | | Missing a required field | `400` | `200`, Customer.io drops the request | | A `type` that doesn't match the endpoint, like `\"type\": \"identify\"` sent to `/track` | `400` | `200`, Customer.io drops the request | | A `context` that isn't an object | `400` | `200`, and we replace `context` with a default object | | A `messageId` longer than 191 characters | `400` | `200`, Customer.io replaces the `messageId` with a hashed ID |  Different endpoints have different required fields:  * Identify, Page, Screen, and Alias calls need a `userId` or an `anonymousId`. * Track calls need a `userId`, an `anonymousId`, or a `groupId`. They also need an `event` name. * Group calls need a `groupId`. * Alias calls need a `previousId`. * Each call in a `/batch` request needs a `type`.  Note that **in strict mode, a `/batch` request succeeds or fails as a whole**. If any call in the batch fails, Customer.io rejects the entire batch with a `400` and doesn't process any of its calls. In lax mode, Customer.io drops the calls that fail and processes the rest.  # Try out our postman collection  We've generated a Postman collection with all of the endpoints organized as you'll find them on this page, with a starter environment (mainly to contain your API key). Set your API key in the environment's `write_key` variable. The collection sends it as a bearer token, which works with a public key, a private key, or a legacy write key.  You'll notice that payloads on this page can contain significantly more information than the payloads that appear in our collection. We've limited our collections to the fields that you'll _typically_ use when you send calls to our APIs and libraries, so it's easier to get started. But you can add additional fields to payloads—like `context`, `integrations`, and so on—if you want.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-287dd370-3d8b-4a71-80fe-75d6b7c7ff61?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-287dd370-3d8b-4a71-80fe-75d6b7c7ff61%26entityType%3Dcollection%26workspaceId%3D35e4a70d-66bd-4b3e-8a0c-57f9e32080dc#?env%5BCustomer.io%20Data%20Pipelines%20API%20Environment%5D=W3sia2V5IjoiY2RwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiY2RwLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJ3cml0ZV9rZXkiLCJ0eXBlIjoic2VjcmV0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfV0=)  # Deletions, suppressions, and other semantic events  You'll notice that this API only contains `POST` calls; we don't have `DELETE` operations. For delete operations, and other operations that don't have bespoke endpoints, we use *semantic events*.  When you need to do things like deleting people, removing relationships, and other sorts of things, you'll send a request to the `/track` endpoint with a specific event `name` parameter. The `name` tells us what to do with the request.  For example, you can send a `track` event with the name `Delete Person` to remove a person from your workspace.  See [Semantic Events](/integrations/data-in/semantic-events/cio-journeys/) to see Customer.io-specific events.  We also support semantic events for other kinds of destinations. See [Semantic Events for other destinations](#semantic-events-for-other-destinations) below for more information.  | Event Name | Description | |------|--------| | `Device Created or Updated` | Adds or updates a mobile device (by token), and associates it with a person | | `Device Deleted` | Deletes a device | | `User Deleted` | Removes a person from your workspace | | `User Suppressed` | Removes a person from your workspace and suppresses their identifiers so you can't add them back to Customer.io. | | `User Unsuppressed` | Unsuppresses an identifier so you can add someone back to Customer.io and message them again. | | `Relationship Deleted` | Remove a relationship between a person and an object| | `Object Deleted` | Remove an object (like an account or company) from your Customer.io workspace| | `Report Delivery Event` | Report delivery events for messages|  ## Semantic events for other destinations  In addition to events that have special meanings [in Customer.io](#deletions-suppressions-and-other-semantic-events), we have a number of other events that we support across different kinds of destinations. These uniform events ensure that we'll map event data to destinations consistently—so you can move from one provider to another without having to change your event names or payload structures.  For example, our ecommerce events work across any ecommerce platform we integrate with—even if those ecommerce platforms have different APIs or payload structures.  See [Semantic Events](/integrations/data-in/semantic-events/getting-started/) to learn more.  * [A/B Test](/integrations/api/cdp/ab-test/) * [Ecommerce](/integrations/api/cdp/ecommerce/) * [Email](/integrations/api/cdp/email/) * [Live Chat](/integrations/api/cdp/live-chat/) * [Mobile App](/integrations/api/cdp/mobile-app/) * [Video](/integrations/api/cdp/video/)  ## Backfilling data  By default, Customer.io records a `timestamp` when we receive requests. If you're sending data to Customer.io in real time, you don't need to worry about the timestamp.  If you want to backfill requests, you can send a `timestamp`—an ISO 8601 date-time string—telling us when the request occurred. This provides a way to log `track` and `page` calls when the activities _actually_ took place. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::pipelines::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchBatchInner {
    /// The kind of request. Required in a `batch` call, where the endpoint can't tell us the type; the single-request endpoints set it for you.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The userId that you want to keep. This is required if you haven't already identified someone with one of our web or server-side libraries. 
    #[serde(rename = "userId")]
    pub user_id: crate::ids::UserId,
    /// A unique substitute for a User ID in cases when you don’t have an absolutely unique identifier. Our libraries generate this value automatically to help you track people before they sign up, log in, provide their email, etc.
    #[serde(rename = "anonymousId")]
    pub anonymous_id: crate::ids::AnonymousId,
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<models::AliasAllContext>>,
    /// Contains a list of booleans indicating the integrations that are enabled (true) or disabled (false). By default, all integrations are enabled (returning an empty object). Set `\"All\": false` to reverse this behavior. 
    #[serde(rename = "integrations", skip_serializing_if = "Option::is_none")]
    pub integrations: Option<std::collections::HashMap<String, bool>>,
    /// A unique identifier for a Pipelines call, ensuring that each individual event is unique. This is set by Customer.io
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// In general, you can use `timestamp` rather than this field if you want to back-date events. This is the timestamp on the client device you invoke a call or the timestamp value you manually passed in a server-side library call.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// The ISO-8601 timestamp when Pipelines receives an event.
    #[serde(rename = "receivedAt", skip_serializing_if = "Option::is_none")]
    pub received_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// The ISO-8601 timestamp when a library sends an event to Pipelines.
    #[serde(rename = "sentAt", skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// The ISO-8601 timestamp when the event originally took place. This is mostly useful when you backfill past events. If you're not backfilling data, you can leave this field empty and we'll use the current time or server time.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<chrono::DateTime<chrono::FixedOffset>>,
    #[serde(rename = "traits", skip_serializing_if = "Option::is_none")]
    pub traits: Option<models::GroupKnownTraits>,
    /// The version of the API that received the event, automatically set by Customer.io.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
    /// The name of the event
    #[serde(rename = "event")]
    pub event: String,
    /// Additional properties for your screen.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// ID of the group
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// The name of the screen the person visited.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The anonymousId or userId value that you want to merge into the canonical profile.
    #[serde(rename = "previousId")]
    pub previous_id: String,
}

impl BatchBatchInner {
    pub fn new(r#type: Type, user_id: crate::ids::UserId, anonymous_id: crate::ids::AnonymousId, event: String, group_id: String, previous_id: String) -> BatchBatchInner {
        BatchBatchInner {
            r#type,
            user_id,
            anonymous_id,
            context: None,
            integrations: None,
            message_id: None,
            original_timestamp: None,
            received_at: None,
            sent_at: None,
            timestamp: None,
            traits: None,
            version: None,
            event,
            properties: None,
            group_id,
            name: None,
            previous_id,
        }
    }
}
/// The kind of request. Required in a `batch` call, where the endpoint can't tell us the type; the single-request endpoints set it for you.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "identify")]
    Identify,
    #[serde(rename = "track")]
    Track,
    #[serde(rename = "page")]
    Page,
    #[serde(rename = "screen")]
    Screen,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "alias")]
    Alias,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for Type {
    fn default() -> Type {
        Self::Identify
    }
}

