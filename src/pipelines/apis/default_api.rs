/*
 * Pipelines API
 *
 * # Overview In general, you'll consume this API through one of our source libraries—our JavaScript client library or any of our server packages. But you can also integrate directly with our REST API if you don't want to install one of our libraries or you want to support a source that we don't have a native integration with.  # Server addresses: US and EU Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://cdp.customer.io | | EU | https://cdp-eu.customer.io |  If you're in our EU region, you'll need to specify the EU URL when you initialize our server-side libraries. If you use our JavaScript client library, we'll set your region and route data/calls automatically.  # Authentication & rate limits  Our API uses basic authorization with the API key from your **Customer.io API** integration. If you use Postman or another platform that helps you send API calls, this API key is the *Username*, and the *Password* is blank.  # Get an API key  You can get a Pipelines API key by [setting up the **Customer.io API** integration](/integrations/data-in/connections/http-api/) in your workspace.  Now you can use the key as a username with a blank password to authenticate and send requests to our API.  ## Rate and payload limits  A request is limited to 32KB. A batch request is limited to 500KB total and 32KB per call in the request. If a request exceeds these limits, Customer.io doesn't process it. By default, Customer.io still returns a `200` response. In [strict mode](#strict-mode), Customer.io returns a `400` instead.  The Data Pipelines API has no fixed rate limit. Customer.io accepts your requests as fast as you send them and doesn't reject or drop them because of volume. Our guideline for both active integrations and historical backfills is about 3000 requests per 3 seconds, an average of 1000 requests per second. The 3-second window leaves room for short bursts.  Traffic above this level usually comes from high-volume integrations, which our Premium and Enterprise plans support. If your traffic regularly goes above this, we may contact you to learn more about your integration and talk about optimizing it, or about a plan that fits your volume.  <div class=\"fly-panel fly-light regionUS\"> <div class=\"fly-panel-body us-server\"> <p class=\"text--bold\">Warning</p> <p>Rate limits are subject to change. We may adjust these thresholds to ensure stable performance for all customers.</p> </div></div>  # Strict mode  The Pipelines API runs in *lax mode* by default. It returns a `200` response for almost every request, and logs issues after the fact in Customer.io. Lax mode keeps a bad request from breaking your integration, but it also means that a `200` response doesn't confirm that Customer.io processed your data.  Turn on *strict mode* to validate each request and return an error when it fails. This can help you build and test an integration, so you see each problem as soon as it happens. A strict mode error response contains an `errors` array. Each error has a `reason`, the `field` that failed, and a `message` that describes the problem. Errors from a `/batch` request also include an `index`, which is the position of the failing call in the `batch` array.  To turn on *strict mode*, send the `X-Strict-Mode` header with a value of `1`. If you leave the header out or send any other value, the request runs in lax mode.  | Problem with the request | Strict mode (`X-Strict-Mode: 1`) | Lax mode (default) | | :-- | :-- | :-- | | Missing or invalid API key | `401` | `200`, Customer.io drops the request | | Disabled API key | `401` | `404` | | A `Content-Encoding` other than `gzip` or `deflate` | `400` | `200`, Customer.io drops the request | | A compressed body that we can't decompress | `500` | `200`, Customer.io drops the request | | Over the [size limits](#rate-and-payload-limits) | `400` | `200`, Customer.io drops the request | | Body that isn't valid JSON | `400` | `200`, Customer.io drops the request | | Missing a required field | `400` | `200`, Customer.io drops the request | | A `type` that doesn't match the endpoint, like `\"type\": \"identify\"` sent to `/track` | `400` | `200`, Customer.io drops the request | | A `context` that isn't an object | `400` | `200`, and we replace `context` with a default object | | A `messageId` longer than 191 characters | `400` | `200`, Customer.io replaces the `messageId` with a hashed ID |  Different endpoints have different required fields:  * Identify, Page, Screen, and Alias calls need a `userId` or an `anonymousId`. * Track calls need a `userId`, an `anonymousId`, or a `groupId`. They also need an `event` name. * Group calls need a `groupId`. * Alias calls need a `previousId`. * Each call in a `/batch` request needs a `type`.  Note that **in strict mode, a `/batch` request succeeds or fails as a whole**. If any call in the batch fails, Customer.io rejects the entire batch with a `400` and doesn't process any of its calls. In lax mode, Customer.io drops the calls that fail and processes the rest.  # Try out our postman collection  We've generated a Postman collection with all of the endpoints organized as you'll find them on this page, with a starter environment (mainly to contain your API key). For our API endpoints, **your API key is your username and your password is blank**.  You'll notice that payloads on this page can contain significantly more information than the payloads that appear in our collection. We've limited our collections to the fields that you'll _typically_ use when you send calls to our APIs and libraries, so it's easier to get started. But you can add additional fields to payloads—like `context`, `integrations`, and so on—if you want.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-287dd370-3d8b-4a71-80fe-75d6b7c7ff61?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-287dd370-3d8b-4a71-80fe-75d6b7c7ff61%26entityType%3Dcollection%26workspaceId%3D35e4a70d-66bd-4b3e-8a0c-57f9e32080dc#?env%5BCustomer.io%20Data%20Pipelines%20API%20Environment%5D=W3sia2V5IjoiY2RwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiY2RwLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJ3cml0ZV9rZXkiLCJ0eXBlIjoic2VjcmV0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfV0=)  # Deletions, suppressions, and other semantic events  You'll notice that this API only contains `POST` calls; we don't have `DELETE` operations. For delete operations, and other operations that don't have bespoke endpoints, we use *semantic events*.  When you need to do things like deleting people, removing relationships, and other sorts of things, you'll send a request to the `/track` endpoint with a specific event `name` parameter. The `name` tells us what to do with the request.  For example, you can send a `track` event with the name `Delete Person` to remove a person from your workspace.  See [Semantic Events](/integrations/data-in/semantic-events/cio-journeys/) to see Customer.io-specific events.  We also support semantic events for other kinds of destinations. See [Semantic Events for other destinations](#semantic-events-for-other-destinations) below for more information.  | Event Name | Description | |------|--------| | `Device Created or Updated` | Adds or updates a mobile device (by token), and associates it with a person | | `Device Deleted` | Deletes a device | | `User Deleted` | Removes a person from your workspace | | `User Suppressed` | Removes a person from your workspace and suppresses their identifiers so you can't add them back to Customer.io. | | `User Unsuppressed` | Unsuppresses an identifier so you can add someone back to Customer.io and message them again. | | `Relationship Deleted` | Remove a relationship between a person and an object| | `Object Deleted` | Remove an object (like an account or company) from your Customer.io workspace| | `Report Delivery Event` | Report delivery events for messages|  ## Semantic events for other destinations  In addition to events that have special meanings [in Customer.io](#deletions-suppressions-and-other-semantic-events), we have a number of other events that we support across different kinds of destinations. These uniform events ensure that we'll map event data to destinations consistently—so you can move from one provider to another without having to change your event names or payload structures.  For example, our ecommerce events work across any ecommerce platform we integrate with—even if those ecommerce platforms have different APIs or payload structures.  See [Semantic Events](/integrations/data-in/semantic-events/getting-started/) to learn more.  * [A/B Test](/integrations/api/cdp/ab-test/) * [Ecommerce](/integrations/api/cdp/ecommerce/) * [Email](/integrations/api/cdp/email/) * [Live Chat](/integrations/api/cdp/live-chat/) * [Mobile App](/integrations/api/cdp/mobile-app/) * [Video](/integrations/api/cdp/video/)  ## Backfilling data  By default, Customer.io records a `timestamp` when we receive requests. If you're sending data to Customer.io in real time, you don't need to worry about the timestamp.  If you want to backfill requests, you can send a `timestamp`—an ISO 8601 date-time string—telling us when the request occurred. This provides a way to log `track` and `page` calls when the activities _actually_ took place. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::pipelines::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`alias`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AliasError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BatchError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`identify`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdentifyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PageError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`screen`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScreenError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`track`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TrackError {
    UnknownValue(serde_json::Value),
}


/// *You **only** need to use this method to support a few select destinations like Mixpanel.*  The `alias` method reconciles identifiers in systems that don't automatically handle identity changes—like when a person graduates from an anonymous user to an identified user.  For example, a person typically has an `anonymousId` until you identify them by `userId`. Most destinations will automatically associate data from the anonymous ID with the new user ID when you send an `identify` call. But a select few destinations, like [Mixpanel](/integrations/data-out/connections/mixpanel/), won't! The `alias` call tells Mixpanel to represent the `anonymousId` with the new `userId`.  If you need to use the `alias` call, you'll want to send it before you identify someone with a `userId`.  If you use our web or server-side libraries, and you've already identified someone, we assume that this is the `userId` that you want to keep. Otherwise, you need to pass both the `previousId` and the `userId`.  **When you use our libraries, you'll typically only provide the `previousId` and `userId`. The libraries fill in the rest of the payload automatically.** 
pub async fn alias(configuration: &configuration::Configuration, x_strict_mode: Option<&str>, alias: Option<models::Alias>) -> Result<(), Error<AliasError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_x_strict_mode = x_strict_mode;
    let p_body_alias = alias;

    let uri_str = format!("{}/alias", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_x_strict_mode {
        req_builder = req_builder.header("X-Strict-Mode", param_value.to_string());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_body_alias);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<AliasError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// The batch method helps you send an array of `identify`, `group`, `track`, `page`, `screen`, and/or `alias` requests in a single call, so you don't have to send multiple requests. Our server-side sources use this method automatically to increase performance.  Requests are limited to 500KB total per request and 32KB per call in the request. A top-level `context` object is the default for every call in the batch: we apply it to each call that doesn't include its own `context`. Set `integrations` on individual calls; a top-level `integrations` object has no effect. 
pub async fn batch(configuration: &configuration::Configuration, x_strict_mode: Option<&str>, batch: Option<models::Batch>) -> Result<(), Error<BatchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_x_strict_mode = x_strict_mode;
    let p_body_batch = batch;

    let uri_str = format!("{}/batch", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_x_strict_mode {
        req_builder = req_builder.header("X-Strict-Mode", param_value.to_string());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_body_batch);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<BatchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Group calls add people to a group. Groups could be a companies, accounts, online classes, and so on. How groups are represented in your destination depends on the destination. Some destinations have concepts of \"groups\"; Customer.io's destination calls groups \"objects.\"  **When you use our libraries, you'll typically only provide an ID for the user, the `groupId`, and `traits`. The libraries fill in the rest of the payload automatically.** 
pub async fn group(configuration: &configuration::Configuration, x_strict_mode: Option<&str>, group: Option<models::Group>) -> Result<(), Error<GroupError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_x_strict_mode = x_strict_mode;
    let p_body_group = group;

    let uri_str = format!("{}/group", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_x_strict_mode {
        req_builder = req_builder.header("X-Strict-Mode", param_value.to_string());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_body_group);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<GroupError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Identifies a person and assigns `traits` to them.  In most cases, you'll identify people with a `userId` or an `anonymousId`. But when you use Customer.io as a destination, you can pass an empty `userId` or `anonymousId`, and identify people by their `email` trait.  **When you use our libraries, you'll typically only provide an ID and `traits`. The libraries fill in the rest of the payload automatically.** 
pub async fn identify(configuration: &configuration::Configuration, x_strict_mode: Option<&str>, identify: Option<models::Identify>) -> Result<(), Error<IdentifyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_x_strict_mode = x_strict_mode;
    let p_body_identify = identify;

    let uri_str = format!("{}/identify", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_x_strict_mode {
        req_builder = req_builder.header("X-Strict-Mode", param_value.to_string());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_body_identify);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<IdentifyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Sends a page view event. If you use our JavaScript source, it automatically records `page` events whenever it loads (every page). If you use a single-page app, you'll need to call the `page` method people change routes.  The request consists of the page `name` and additional properties about the page.  If you use our JavaScript library, the page name and URL are automatically gathered and passed as event properties.  **When you use our libraries, you'll typically only provide a user ID/anonymous ID and the `name` of the page. The libraries fill in the rest of the payload automatically.** 
pub async fn page(configuration: &configuration::Configuration, x_strict_mode: Option<&str>, page_request: Option<models::PageRequest>) -> Result<(), Error<PageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_x_strict_mode = x_strict_mode;
    let p_body_page_request = page_request;

    let uri_str = format!("{}/page", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_x_strict_mode {
        req_builder = req_builder.header("X-Strict-Mode", param_value.to_string());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_body_page_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<PageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Sends a screen view event for mobile devices. Screen events help you understand the screens that people use in your app. You should send a screen call whenever a user requests a page of your app. 
pub async fn screen(configuration: &configuration::Configuration, x_strict_mode: Option<&str>, screen_request: Option<models::ScreenRequest>) -> Result<(), Error<ScreenError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_x_strict_mode = x_strict_mode;
    let p_body_screen_request = screen_request;

    let uri_str = format!("{}/screen", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_x_strict_mode {
        req_builder = req_builder.header("X-Strict-Mode", param_value.to_string());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_body_screen_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ScreenError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Send an event associated with a person. The `track` call is how you record the things that people do on your website or in your app.  While this endpoint takes any `event` name you want to send, Customer.io also supports \"semantic events.\" These are specific `event` names (like `User Deleted` or `Device Created or Updated`) that perform special operations in Customer.io like deleting people, adding mobile devices to profiles, and so on. See the [Customer.io Semantic Events documentation](/integrations/api/cdp/customerio/) for complete schemas and examples.  **When you use our libraries, you'll typically only provide an `event`, an ID for the person, and `properties`. The libraries fill in the rest of the payload automatically.** 
pub async fn track(configuration: &configuration::Configuration, x_strict_mode: Option<&str>, track_request: Option<models::TrackRequest>) -> Result<(), Error<TrackError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_x_strict_mode = x_strict_mode;
    let p_body_track_request = track_request;

    let uri_str = format!("{}/track", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_x_strict_mode {
        req_builder = req_builder.header("X-Strict-Mode", param_value.to_string());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_body_track_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<TrackError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

