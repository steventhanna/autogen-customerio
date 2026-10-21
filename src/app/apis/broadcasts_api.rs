/*
 * Customer.io App API
 *
 * Our App API provides ways to trigger messages and retrieve information about people, automations, broadcasts, and more.  # Overview  The App API provides methods to send newsletters, transactional messages, and API-triggered broadcasts. You can create newsletters from scratch and update transactional messages and API-triggered broadcasts.  For transactional messages and API-triggered broadcasts, your payload acts as a message \"trigger\" and can contain `data` that you reference in your messages using liquid—`{{trigger.<data>}}`.  The other endpoints help you retrieve information about people, segments, automations, broadcasts, etc; it also lets you update automation actions, messages, newsletter variants, etc. Aside from the [API-triggered broadcast](/integrations/api/app/tag/send-messages/triggerBroadcast/) endpoint (1 per 10 seconds), requests are limited to 10 per second. [Transactional](/integrations/api/app/tag/send-messages/sendEmail/) messages aren't rate limited.  # We may add to these APIs  As we release new features, we add endpoints and we may even add fields to existing endpoints. We don't announce these additions in advance. Fields we add will be backward compatible: existing fields keep their names and types.  Build your integration to ignore fields it doesn't recognize. If you validate responses against a schema, make sure the schema accepts unknown keys. In JSON Schema, for example, don't set `additionalProperties: false`.  # Use our Postman collection  We've generated a Postman collection to help you get started with our APIs.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  **NOTE**: Postman endpoints default to our US APIs. If you're in our European (EU) region, you'll need to add `-eu` to the server variables (`track_api_url` and `app_api_url`).  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d%26entityType%3Dcollection%26workspaceId%3Db886877f-fc09-475f-84fe-6221a98f4d18#?env%5BCustomer.io%20API%20Environment%5D=W3sia2V5IjoidHJhY2tfYXBpX3VybCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiJ0cmFjay5jdXN0b21lci5pbyIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiYXBpLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzaXRlX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBpX2tleSIsInR5cGUiOiJzZWNyZXQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYmVhcmVyIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYnJvYWRjYXN0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaW1wb3J0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiZW1haWxfYWRkcmVzcyIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN1cHByZXNzaW9uX3R5cGUiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjb2xsZWN0aW9uX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoic25pcHBldF9uYW1lIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoid2ViaG9va19pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InNlbmRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImV4cG9ydF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6Im1lc3NhZ2VfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzZWdtZW50X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoibmV3c2xldHRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImNvbnRlbnRfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjYW1wYWlnbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImFjdGlvbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImN1c3RvbWVyX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoidHJhbnNhY3Rpb25hbF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRyaWdnZXJfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJmb3JtX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaWRlbnRpZmllciIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRldmljZV9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRlbGl2ZXJ5X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9XQ==)  # Server addresses: US and EU  Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://api.customer.io | | EU | https://api-eu.customer.io |  # Authentication  All requests to the Customer.io App API use an [App API Key](#authentication).  To authenticate, provide your key as a Bearer token in a HTTP Authorization header. You can create and manage your API keys—including keys with different scopes—in [your account settings page](https://fly.customer.io/settings/api_credentials?keyType=app). Each operation on this page references the authorization header it requires.  # Rate Limits  Most endpoints on this page are limited to 10 requests per second. The exceptions are: * [Transactional messages](/integrations/api/app/tag/send-messages/sendEmail/) aren't rate limited. * The [API-triggered broadcast endpoint](/integrations/api/app/tag/send-messages/triggerBroadcast/) is limited to 1 request every 10 seconds. If you need to trigger broadcasts more often, contact us to talk through your use case.  **Rate limits are subject to change. We may adjust these thresholds to ensure stable performance for all customers.** 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::app::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`broadcast_action_links`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastActionLinksError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_action_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastActionMetricsError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_actions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastActionsError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_errors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastErrorsError {
    Status400(),
    Status401(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_links`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastLinksError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastMessagesError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastMetricsError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastStatusError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_broadcast`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBroadcastError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_broadcast_action`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBroadcastActionError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_broadcast_action_language`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBroadcastActionLanguageError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_broadcast_triggers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListBroadcastTriggersError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_broadcasts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListBroadcastsError {
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_broadcast_action`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateBroadcastActionError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_broadcast_action_language`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateBroadcastActionLanguageError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}


/// Returns link click metrics for an individual broadcast action. Unless you specify otherwise, the response contains data for the maximum period by days (45 days).  You cannot request fewer than 2 steps of any period (2 hours, 2 days, 2 weeks, or 2 months). For instance, `?period=days&steps=1` means two days - the 48 hours before the API request was made. `?period=days&steps=0` returns the same as the maximum of the period - `?period=days&steps=45`. See the `steps` parameter below for the maximum count of each period. 
pub async fn broadcast_action_links(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId, action_id: crate::ids::ActionId, period: Option<&str>, steps: Option<i64>, r#type: Option<&str>) -> Result<models::CampaignLinkMetrics200Response, Error<BroadcastActionLinksError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;
    let p_path_action_id = action_id;
    let p_query_period = period;
    let p_query_steps = steps;
    let p_query_type = r#type;

    let uri_str = format!("{}/v1/broadcasts/{broadcast_id}/actions/{action_id}/metrics/links", configuration.base_path, broadcast_id=p_path_broadcast_id, action_id=p_path_action_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_period {
        req_builder = req_builder.query(&[("period", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_steps {
        req_builder = req_builder.query(&[("steps", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CampaignLinkMetrics200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CampaignLinkMetrics200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BroadcastActionLinksError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a list of metrics for an individual action both in total and in `steps` (days, weeks, etc) over a period of time. Stepped `series` metrics return from oldest to newest (i.e. the 0-index for any result is the oldest step/period).  You cannot request fewer than 2 steps of any period (2 hours, 2 days, 2 weeks, or 2 months). For instance, `?period=days&steps=1` means two days - the 48 hours before the API request was made. `?period=days&steps=0` returns the same as the maximum of the period - `?period=days&steps=45`. See the `steps` parameter below for the maximum count of each period.  **Multi-language messages**  If the action is a [multi-language](/messaging/channels/localization/getting-started/) message, you can retrieve metrics for a single language variant or all language variants. To retrieve metrics for a single language variant, pass the `id` of the action. To retrieve aggregate metrics and a breakdown for each language variant, pass the `multi_language_branch_action_id`, which is the same for each variant of a message. You can get both ids from the [List broadcast actions](/integrations/api/app/tag/broadcasts/broadcastActions/) endpoint.  If you pass the `multi_language_branch_action_id`, then the `metric` object shows the *total* across every variant. The `language_variants` object shows a breakdown for each language variant.  If you want to get the sum of all metrics across all of a broadcast's actions, you can use each action id of a multi-language message, like you would for other types of actions. 
pub async fn broadcast_action_metrics(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId, action_id: crate::ids::ActionId, period: Option<&str>, steps: Option<i64>, r#type: Option<&str>) -> Result<models::BroadcastActionMetrics200Response, Error<BroadcastActionMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;
    let p_path_action_id = action_id;
    let p_query_period = period;
    let p_query_steps = steps;
    let p_query_type = r#type;

    let uri_str = format!("{}/v1/broadcasts/{broadcast_id}/actions/{action_id}/metrics", configuration.base_path, broadcast_id=p_path_broadcast_id, action_id=p_path_action_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_period {
        req_builder = req_builder.query(&[("period", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_steps {
        req_builder = req_builder.query(&[("steps", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BroadcastActionMetrics200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BroadcastActionMetrics200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BroadcastActionMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the actions that occur as a part of a broadcast.  If a message block contains multiple languages, this endpoint returns an object for each language variant. Each language variant includes the `multi_language_branch_action_id` property with the same value. Use this id to retrieve metrics for each language variant in a single request through [Get broadcast action metrics](/integrations/api/app/tag/broadcasts/broadcastActionMetrics/). 
pub async fn broadcast_actions(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId) -> Result<models::BroadcastActions200Response, Error<BroadcastActionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;

    let uri_str = format!("{}/v1/broadcasts/{broadcast_id}/actions", configuration.base_path, broadcast_id=p_path_broadcast_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BroadcastActions200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BroadcastActions200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BroadcastActionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// If your broadcast produced validation errors, this endpoint can help you better understand what went wrong. Broadcast errors are generally issues in your broadcast audience and associated. 
pub async fn broadcast_errors(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId, trigger_id: crate::ids::TriggerId, start: Option<&str>, limit: Option<i64>) -> Result<models::BroadcastErrors200Response, Error<BroadcastErrorsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;
    let p_path_trigger_id = trigger_id;
    let p_query_start = start;
    let p_query_limit = limit;

    let uri_str = format!("{}/v1/campaigns/{broadcast_id}/triggers/{trigger_id}/errors", configuration.base_path, broadcast_id=p_path_broadcast_id, trigger_id=p_path_trigger_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_start {
        req_builder = req_builder.query(&[("start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BroadcastErrors200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BroadcastErrors200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BroadcastErrorsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns metrics for link clicks within a broadcast, both in total and in `series` periods (days, weeks, etc). `series` metrics are ordered oldest to newest (i.e. the 0-index for any result is the oldest step/period).  You cannot request fewer than 2 steps of any period (2 hours, 2 days, 2 weeks, or 2 months). For instance, `?period=days&steps=1` means two days - the 48 hours before the API request was made. `?period=days&steps=0` returns the same as the maximum of the period - `?period=days&steps=45`. See the `steps` parameter below for the maximum count of each period. 
pub async fn broadcast_links(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId, period: Option<&str>, steps: Option<i64>, unique: Option<bool>) -> Result<models::CampaignLinkMetrics200Response, Error<BroadcastLinksError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;
    let p_query_period = period;
    let p_query_steps = steps;
    let p_query_unique = unique;

    let uri_str = format!("{}/v1/broadcasts/{broadcast_id}/metrics/links", configuration.base_path, broadcast_id=p_path_broadcast_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_period {
        req_builder = req_builder.query(&[("period", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_steps {
        req_builder = req_builder.query(&[("steps", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_unique {
        req_builder = req_builder.query(&[("unique", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CampaignLinkMetrics200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CampaignLinkMetrics200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BroadcastLinksError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns information about the deliveries (instances of messages sent to individual people) sent from an API-triggered broadcast. Provide query parameters to refine the metrics you want to return.  Use the `start_ts` and `end_ts` to find messages within a time range. If your request doesn't include `start_ts` and `end_ts` parameters, we'll return results for the 1 month period after the first trigger. If your `start_ts` and `end_ts` range is more than 12 months, we'll return 12 months of data from the most recent timestamp in your request. Timestamps reflect when deliveries were created in our system, not when they were actually sent to recipients. There may be a delay between creation and sending. 
pub async fn broadcast_messages(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId, start: Option<&str>, limit: Option<i64>, metric: Option<&str>, state: Option<&str>, r#type: Option<&str>, start_ts: Option<i64>, end_ts: Option<i64>, get_tracked_responses: Option<bool>) -> Result<models::GetCampaignMessages200Response, Error<BroadcastMessagesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;
    let p_query_start = start;
    let p_query_limit = limit;
    let p_query_metric = metric;
    let p_query_state = state;
    let p_query_type = r#type;
    let p_query_start_ts = start_ts;
    let p_query_end_ts = end_ts;
    let p_query_get_tracked_responses = get_tracked_responses;

    let uri_str = format!("{}/v1/broadcasts/{broadcast_id}/messages", configuration.base_path, broadcast_id=p_path_broadcast_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_start {
        req_builder = req_builder.query(&[("start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_metric {
        req_builder = req_builder.query(&[("metric", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_state {
        req_builder = req_builder.query(&[("state", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_ts {
        req_builder = req_builder.query(&[("start_ts", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_ts {
        req_builder = req_builder.query(&[("end_ts", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_get_tracked_responses {
        req_builder = req_builder.query(&[("get_tracked_responses", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCampaignMessages200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCampaignMessages200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BroadcastMessagesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a list of metrics for an individual broadcast in `steps` (days, weeks, etc). We return metrics from oldest to newest (i.e. the 0-index for any result is the oldest step/period).  You cannot request fewer than 2 steps of any period (2 hours, 2 days, 2 weeks, or 2 months). For instance, `?period=days&steps=1` means two days - the 48 hours before the API request was made. `?period=days&steps=0` returns the same as the maximum of the period - `?period=days&steps=45`. See the `steps` parameter below for the maximum count of each period. 
pub async fn broadcast_metrics(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId, period: Option<&str>, steps: Option<i64>, r#type: Option<&str>) -> Result<models::BroadcastMetrics200Response, Error<BroadcastMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;
    let p_query_period = period;
    let p_query_steps = steps;
    let p_query_type = r#type;

    let uri_str = format!("{}/v1/broadcasts/{broadcast_id}/metrics", configuration.base_path, broadcast_id=p_path_broadcast_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_period {
        req_builder = req_builder.query(&[("period", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_steps {
        req_builder = req_builder.query(&[("steps", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BroadcastMetrics200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BroadcastMetrics200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BroadcastMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// After triggering a broadcast you can retrieve the status of that broadcast using a GET of the `trigger_id`. You can retrieve the `trigger_id` from [Get broadcast triggers](/integrations/api/app/tag/broadcasts/listBroadcastTriggers/). 
pub async fn broadcast_status(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId, trigger_id: crate::ids::TriggerId) -> Result<models::BroadcastStatus200Response, Error<BroadcastStatusError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;
    let p_path_trigger_id = trigger_id;

    let uri_str = format!("{}/v1/campaigns/{broadcast_id}/triggers/{trigger_id}", configuration.base_path, broadcast_id=p_path_broadcast_id, trigger_id=p_path_trigger_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::BroadcastStatus200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::BroadcastStatus200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BroadcastStatusError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns metadata for an individual broadcast.
pub async fn get_broadcast(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId) -> Result<models::GetBroadcast200Response, Error<GetBroadcastError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;

    let uri_str = format!("{}/v1/broadcasts/{broadcast_id}", configuration.base_path, broadcast_id=p_path_broadcast_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetBroadcast200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetBroadcast200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetBroadcastError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns information about a specific action within a broadcast.
pub async fn get_broadcast_action(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId, action_id: crate::ids::ActionId) -> Result<models::GetBroadcastAction200Response, Error<GetBroadcastActionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;
    let p_path_action_id = action_id;

    let uri_str = format!("{}/v1/broadcasts/{broadcast_id}/actions/{action_id}", configuration.base_path, broadcast_id=p_path_broadcast_id, action_id=p_path_action_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetBroadcastAction200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetBroadcastAction200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetBroadcastActionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns information about a translation of message in a broadcast. The message is identified by the `action_id`. 
pub async fn get_broadcast_action_language(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId, action_id: crate::ids::ActionId, language: &str) -> Result<models::GetBroadcastAction200Response, Error<GetBroadcastActionLanguageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;
    let p_path_action_id = action_id;
    let p_path_language = language;

    let uri_str = format!("{}/v1/broadcasts/{broadcast_id}/actions/{action_id}/language/{language}", configuration.base_path, broadcast_id=p_path_broadcast_id, action_id=p_path_action_id, language=crate::app::apis::urlencode(p_path_language));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetBroadcastAction200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetBroadcastAction200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetBroadcastActionLanguageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a list of the `triggers` for a broadcast.
pub async fn list_broadcast_triggers(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId) -> Result<models::ListBroadcastTriggers200Response, Error<ListBroadcastTriggersError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;

    let uri_str = format!("{}/v1/broadcasts/{broadcast_id}/triggers", configuration.base_path, broadcast_id=p_path_broadcast_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListBroadcastTriggers200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListBroadcastTriggers200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListBroadcastTriggersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a list of your API-triggered broadcasts and associated metadata.
pub async fn list_broadcasts(configuration: &configuration::Configuration, ) -> Result<models::ListBroadcasts200Response, Error<ListBroadcastsError>> {

    let uri_str = format!("{}/v1/broadcasts", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListBroadcasts200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListBroadcasts200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListBroadcastsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update the contents of a broadcast action, including the body of messages or HTTP requests.   **NOTE** You cannot manage content made with Design Studio with this endpoint. Use the [Design Studio APIs](/integrations/api/design-studio/) instead. 
pub async fn update_broadcast_action(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId, action_id: crate::ids::ActionId, update_broadcast_action_object: Option<models::UpdateBroadcastActionObject>) -> Result<models::GetBroadcastAction200Response, Error<UpdateBroadcastActionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;
    let p_path_action_id = action_id;
    let p_body_update_broadcast_action_object = update_broadcast_action_object;

    let uri_str = format!("{}/v1/broadcasts/{broadcast_id}/actions/{action_id}", configuration.base_path, broadcast_id=p_path_broadcast_id, action_id=p_path_action_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_broadcast_action_object);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetBroadcastAction200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetBroadcastAction200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateBroadcastActionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update a translation of a specific broadcast action, including the body of messages or HTTP requests.  **NOTE** You cannot manage content made with Design Studio with this endpoint. Use the [Design Studio APIs](/integrations/api/design-studio/) instead. 
pub async fn update_broadcast_action_language(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId, action_id: crate::ids::ActionId, language: &str, update_broadcast_action_object: Option<models::UpdateBroadcastActionObject>) -> Result<models::GetBroadcastAction200Response, Error<UpdateBroadcastActionLanguageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;
    let p_path_action_id = action_id;
    let p_path_language = language;
    let p_body_update_broadcast_action_object = update_broadcast_action_object;

    let uri_str = format!("{}/v1/broadcasts/{broadcast_id}/actions/{action_id}/language/{language}", configuration.base_path, broadcast_id=p_path_broadcast_id, action_id=p_path_action_id, language=crate::app::apis::urlencode(p_path_language));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_broadcast_action_object);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetBroadcastAction200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetBroadcastAction200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateBroadcastActionLanguageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

