/*
 * Pipelines API
 *
 * # Overview In general, you'll consume this API through one of our source libraries—our JavaScript client library or any of our server packages. But you can also integrate directly with our REST API if you don't want to install one of our libraries or you want to support a source that we don't have a native integration with.  # Server addresses: US and EU Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://cdp.customer.io | | EU | https://cdp-eu.customer.io |  If you're in our EU region, you'll need to specify the EU URL when you initialize our server-side libraries. If you use our JavaScript client library, we'll set your region and route data/calls automatically.  # Authentication & rate limits  Our API uses basic authorization with the API key from your **Customer.io API** integration. If you use Postman or another platform that helps you send API calls, this API key is the *Username*, and the *Password* is blank.  # Get an API key  You can get a Pipelines API key by [setting up the **Customer.io API** integration](/integrations/data-in/connections/http-api/) in your workspace.  Now you can use the key as a username with a blank password to authenticate and send requests to our API.  ## Rate and payload limits  A request is limited to 32KB. A batch request is limited to 500KB total and 32KB per call in the request. If a request exceeds these limits, Customer.io doesn't process it. By default, Customer.io still returns a `200` response. In [strict mode](#strict-mode), Customer.io returns a `400` instead.  The Pipelines API has no fixed rate limit. Customer.io accepts your requests as fast as you send them and doesn't reject or drop them because of volume. Our guideline for both active integrations and historical backfills is about 3000 requests per 3 seconds, an average of 1000 requests per second. The 3-second window leaves room for short bursts.  Traffic above this level usually comes from high-volume integrations, which our Premium and Enterprise plans support. If your traffic regularly goes above this, we may contact you to learn more about your integration and talk about optimizing it, or about a plan that fits your volume.  <div class=\"fly-panel fly-light regionUS\"> <div class=\"fly-panel-body us-server\"> <p class=\"text--bold\">Warning</p> <p>Rate limits are subject to change. We may adjust these thresholds to ensure stable performance for all customers.</p> </div></div>  # Strict mode  The Pipelines API runs in *lax mode* by default. It returns a `200` response for almost every request, and logs issues after the fact in Customer.io. Lax mode keeps a bad request from breaking your integration, but it also means that a `200` response doesn't confirm that Customer.io processed your data.  Turn on *strict mode* to validate each request and return an error when it fails. This can help you build and test an integration, so you see each problem as soon as it happens. A strict mode error response contains an `errors` array. Each error has a `reason`, the `field` that failed, and a `message` that describes the problem. Errors from a `/batch` request also include an `index`, which is the position of the failing call in the `batch` array.  To turn on *strict mode*, send the `X-Strict-Mode` header with a value of `1`. If you leave the header out or send any other value, the request runs in lax mode.  | Problem with the request | Strict mode (`X-Strict-Mode: 1`) | Lax mode (default) | | :-- | :-- | :-- | | Missing or invalid API key | `401` | `200`, Customer.io drops the request | | Disabled API key | `401` | `404` | | A `Content-Encoding` other than `gzip` or `deflate` | `400` | `200`, Customer.io drops the request | | A compressed body that we can't decompress | `500` | `200`, Customer.io drops the request | | Over the [size limits](#rate-and-payload-limits) | `400` | `200`, Customer.io drops the request | | Body that isn't valid JSON | `400` | `200`, Customer.io drops the request | | Missing a required field | `400` | `200`, Customer.io drops the request | | A `type` that doesn't match the endpoint, like `\"type\": \"identify\"` sent to `/track` | `400` | `200`, Customer.io drops the request | | A `context` that isn't an object | `400` | `200`, and we replace `context` with a default object | | A `messageId` longer than 191 characters | `400` | `200`, Customer.io replaces the `messageId` with a hashed ID |  Different endpoints have different required fields:  * Identify, Page, Screen, and Alias calls need a `userId` or an `anonymousId`. * Track calls need a `userId`, an `anonymousId`, or a `groupId`. They also need an `event` name. * Group calls need a `groupId`. * Alias calls need a `previousId`. * Each call in a `/batch` request needs a `type`.  Note that **in strict mode, a `/batch` request succeeds or fails as a whole**. If any call in the batch fails, Customer.io rejects the entire batch with a `400` and doesn't process any of its calls. In lax mode, Customer.io drops the calls that fail and processes the rest.  # Try out our postman collection  We've generated a Postman collection with all of the endpoints organized as you'll find them on this page, with a starter environment (mainly to contain your API key). For our API endpoints, **your API key is your username and your password is blank**.  You'll notice that payloads on this page can contain significantly more information than the payloads that appear in our collection. We've limited our collections to the fields that you'll _typically_ use when you send calls to our APIs and libraries, so it's easier to get started. But you can add additional fields to payloads—like `context`, `integrations`, and so on—if you want.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-287dd370-3d8b-4a71-80fe-75d6b7c7ff61?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-287dd370-3d8b-4a71-80fe-75d6b7c7ff61%26entityType%3Dcollection%26workspaceId%3D35e4a70d-66bd-4b3e-8a0c-57f9e32080dc#?env%5BCustomer.io%20Data%20Pipelines%20API%20Environment%5D=W3sia2V5IjoiY2RwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiY2RwLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJ3cml0ZV9rZXkiLCJ0eXBlIjoic2VjcmV0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfV0=)  # Deletions, suppressions, and other semantic events  You'll notice that this API only contains `POST` calls; we don't have `DELETE` operations. For delete operations, and other operations that don't have bespoke endpoints, we use *semantic events*.  When you need to do things like deleting people, removing relationships, and other sorts of things, you'll send a request to the `/track` endpoint with a specific event `name` parameter. The `name` tells us what to do with the request.  For example, you can send a `track` event with the name `Delete Person` to remove a person from your workspace.  See [Semantic Events](/integrations/data-in/semantic-events/cio-journeys/) to see Customer.io-specific events.  We also support semantic events for other kinds of destinations. See [Semantic Events for other destinations](#semantic-events-for-other-destinations) below for more information.  | Event Name | Description | |------|--------| | `Device Created or Updated` | Adds or updates a mobile device (by token), and associates it with a person | | `Device Deleted` | Deletes a device | | `User Deleted` | Removes a person from your workspace | | `User Suppressed` | Removes a person from your workspace and suppresses their identifiers so you can't add them back to Customer.io. | | `User Unsuppressed` | Unsuppresses an identifier so you can add someone back to Customer.io and message them again. | | `Relationship Deleted` | Remove a relationship between a person and an object| | `Object Deleted` | Remove an object (like an account or company) from your Customer.io workspace| | `Report Delivery Event` | Report delivery events for messages|  ## Semantic events for other destinations  In addition to events that have special meanings [in Customer.io](#deletions-suppressions-and-other-semantic-events), we have a number of other events that we support across different kinds of destinations. These uniform events ensure that we'll map event data to destinations consistently—so you can move from one provider to another without having to change your event names or payload structures.  For example, our ecommerce events work across any ecommerce platform we integrate with—even if those ecommerce platforms have different APIs or payload structures.  See [Semantic Events](/integrations/data-in/semantic-events/getting-started/) to learn more.  * [A/B Test](/integrations/api/cdp/ab-test/) * [Ecommerce](/integrations/api/cdp/ecommerce/) * [Email](/integrations/api/cdp/email/) * [Live Chat](/integrations/api/cdp/live-chat/) * [Mobile App](/integrations/api/cdp/mobile-app/) * [Video](/integrations/api/cdp/video/)  ## Backfilling data  By default, Customer.io records a `timestamp` when we receive requests. If you're sending data to Customer.io in real time, you don't need to worry about the timestamp.  If you want to backfill requests, you can send a `timestamp`—an ISO 8601 date-time string—telling us when the request occurred. This provides a way to log `track` and `page` calls when the activities _actually_ took place. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::pipelines::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoContentStarted {
    /// The ISO-8601 date-time when the video content/pod originally aired or was published.
    #[serde(rename = "airdate", skip_serializing_if = "Option::is_none")]
    pub airdate: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// The ID of the video a user interacted with.
    #[serde(rename = "asset_id", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// The current kbps.
    #[serde(rename = "bitrate", skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// The channel the video content/pod belongs to or is aired on, like `hgtv` or `my_youtube_channel`.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// The description of the video content/pod.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The average frames per second (fps).
    #[serde(rename = "framerate", skip_serializing_if = "Option::is_none")]
    pub framerate: Option<i64>,
    /// Set to `true` if the content is a full episode of a show.
    #[serde(rename = "full_episode", skip_serializing_if = "Option::is_none")]
    pub full_episode: Option<bool>,
    /// The genre of the video content/pod.
    #[serde(rename = "genre", skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    /// Keywords associated with the video content/pod.
    #[serde(rename = "keywords", skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// Set to `true` if the content is a livestream.
    #[serde(rename = "livestream", skip_serializing_if = "Option::is_none")]
    pub livestream: Option<bool>,
    /// The ID of the content \"pod\" in a playback session. Imagine a video that contains content and an ad; that means the video contains two pods—one for the content and one for the ad. This is the specific piece of content that the user interacted with.
    #[serde(rename = "pod_id", skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,
    /// The current index position in **seconds** of the playhead, including the duration of any ads seen (if available). If the playback is a livestream, check the documentation for relevant destinations for details on how to correctly pass the playhead position.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    /// The name of the program or show the content belongs to (if applicable).
    #[serde(rename = "program", skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
    /// The publisher of the video content/pod.
    #[serde(rename = "publisher", skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    /// The season the video content/pod belongs to, if applicable.
    #[serde(rename = "season", skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,
    /// The unique ID of the overall session used to tie all events generated from a specific playback. This value should be the same across all playback, content, and ad events if they are from the same playback session.
    #[serde(rename = "session_id", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// The title of the video content/pod.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The total duration of the content/asset in seconds. Note that this is not the duration of the video itself or the playback, but a piece of a complete \"pod\" of content. If the content is an ad in the video, this is the length of the ad.
    #[serde(rename = "total_length", skip_serializing_if = "Option::is_none")]
    pub total_length: Option<i64>,
}

impl VideoContentStarted {
    pub fn new() -> VideoContentStarted {
        VideoContentStarted {
            airdate: None,
            asset_id: None,
            bitrate: None,
            channel: None,
            description: None,
            framerate: None,
            full_episode: None,
            genre: None,
            keywords: None,
            livestream: None,
            pod_id: None,
            position: None,
            program: None,
            publisher: None,
            season: None,
            session_id: None,
            title: None,
            total_length: None,
        }
    }
}

