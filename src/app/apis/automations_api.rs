/*
 * Customer.io App API
 *
 * Our App API provides ways to trigger messages and retrieve information about people, automations, broadcasts, and more.  # Overview  The App API provides methods to send newsletters, transactional messages, and API-triggered broadcasts. You can create newsletters from scratch and update transactional messages and API-triggered broadcasts.  For transactional messages and API-triggered broadcasts, your payload acts as a message \"trigger\" and can contain `data` that you reference in your messages using liquid—`{{trigger.<data>}}`.  The other endpoints help you retrieve information about people, segments, automations, broadcasts, etc; it also lets you update automation actions, messages, newsletter variants, etc. Aside from the [API-triggered broadcast](/integrations/api/app/tag/send-messages/triggerBroadcast/) endpoint (1 per 10 seconds), requests are limited to 10 per second. [Transactional](/integrations/api/app/tag/send-messages/sendEmail/) messages aren't rate limited.  # We may add to these APIs  As we release new features, we add endpoints and we may even add fields to existing endpoints. We don't announce these additions in advance. Fields we add will be backward compatible: existing fields keep their names and types.  Build your integration to ignore fields it doesn't recognize. If you validate responses against a schema, make sure the schema accepts unknown keys. In JSON Schema, for example, don't set `additionalProperties: false`.  # Use our Postman collection  We've generated a Postman collection to help you get started with our APIs.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  **NOTE**: Postman endpoints default to our US APIs. If you're in our European (EU) region, you'll need to add `-eu` to the server variables (`track_api_url` and `app_api_url`).  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d%26entityType%3Dcollection%26workspaceId%3Db886877f-fc09-475f-84fe-6221a98f4d18#?env%5BCustomer.io%20API%20Environment%5D=W3sia2V5IjoidHJhY2tfYXBpX3VybCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiJ0cmFjay5jdXN0b21lci5pbyIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiYXBpLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzaXRlX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBpX2tleSIsInR5cGUiOiJzZWNyZXQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYmVhcmVyIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYnJvYWRjYXN0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaW1wb3J0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiZW1haWxfYWRkcmVzcyIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN1cHByZXNzaW9uX3R5cGUiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjb2xsZWN0aW9uX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoic25pcHBldF9uYW1lIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoid2ViaG9va19pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InNlbmRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImV4cG9ydF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6Im1lc3NhZ2VfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzZWdtZW50X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoibmV3c2xldHRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImNvbnRlbnRfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjYW1wYWlnbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImFjdGlvbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImN1c3RvbWVyX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoidHJhbnNhY3Rpb25hbF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRyaWdnZXJfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJmb3JtX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaWRlbnRpZmllciIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRldmljZV9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRlbGl2ZXJ5X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9XQ==)  # Server addresses: US and EU  Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://api.customer.io | | EU | https://api-eu.customer.io |  # Authentication  All requests to the Customer.io App API use a private key (`sk_`) as a bearer token in the HTTP `Authorization` header: `Authorization: Bearer <key>`.  Create a private key on the [API Credentials page](https://fly.customer.io/settings/api_credentials?keyType=app). [Learn more about public and private keys](/accounts/settings/managing-credentials/#public-and-private-keys).  If you already use an App API key, you can keep using it with bearer authentication.  # Rate Limits  Most endpoints on this page are limited to 10 requests per second. The exceptions are: * [Transactional messages](/integrations/api/app/tag/send-messages/sendEmail/) aren't rate limited. * The [API-triggered broadcast endpoint](/integrations/api/app/tag/send-messages/triggerBroadcast/) is limited to 1 request every 10 seconds. If you need to trigger broadcasts more often, contact us to talk through your use case.  **Rate limits are subject to change. We may adjust these thresholds to ensure stable performance for all customers.** 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::app::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`campaign_action_links`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CampaignActionLinksError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`campaign_action_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CampaignActionMetricsError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`campaign_journey_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CampaignJourneyMetricsError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`campaign_link_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CampaignLinkMetricsError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`campaign_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CampaignMetricsError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_campaign_action`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCampaignActionError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_campaign_action_translation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCampaignActionTranslationError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_campaign_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCampaignMessagesError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_campaigns`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCampaignsError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_campaign_actions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCampaignActionsError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_campaigns`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCampaignsError {
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_campaign_action`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCampaignActionError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_campaign_action_translation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCampaignActionTranslationError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}


/// Returns link click metrics for an individual action. Unless you specify otherwise, the response contains data for the maximum period by days (45 days).  You cannot request fewer than 2 steps of any period (2 hours, 2 days, 2 weeks, or 2 months). For instance, `?period=days&steps=1` means two days - the 48 hours before the API request was made. `?period=days&steps=0` returns the same as the maximum of the period - `?period=days&steps=45`. See the `steps` parameter below for the maximum count of each period. 
pub async fn campaign_action_links(configuration: &configuration::Configuration, campaign_id: crate::ids::CampaignId, action_id: crate::ids::ActionId, period: Option<&str>, steps: Option<i64>, r#type: Option<&str>, unique: Option<bool>) -> Result<models::CampaignLinkMetrics200Response, Error<CampaignActionLinksError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_campaign_id = campaign_id;
    let p_path_action_id = action_id;
    let p_query_period = period;
    let p_query_steps = steps;
    let p_query_type = r#type;
    let p_query_unique = unique;

    let uri_str = format!("{}/v1/campaigns/{campaign_id}/actions/{action_id}/metrics/links", configuration.base_path, campaign_id=p_path_campaign_id, action_id=p_path_action_id);
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
        let entity: Option<CampaignActionLinksError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns metrics for an individual action, both in total and as a series of counts, oldest first—the 0-index of any result is the oldest point in the window.  **We recommend version 2. Send `version=2`.** It accepts an explicit `start`, `end`, and `res`, reported in the `tz` you choose. Without it, the endpoint answers with version 1 behavior instead: a window built from `period` and `steps`, always in Eastern time.  Version 1 can't return fewer than 2 steps of any period. `?period=days&steps=1` means two days—the 48 hours before the request—and `?period=days&steps=0` returns the maximum for the period, the same as `?period=days&steps=45`.  **Multi-language messages**  If the action is a [multi-language](/messaging/channels/localization/getting-started/) message, you can retrieve metrics for a single language variant or all language variants. To retrieve metrics for a single language variant, pass the `id` of the action. To retrieve aggregate metrics and a breakdown for each language variant, pass the `multi_language_branch_action_id`, which is the same for each variant of a message. You can get both ids from the [List automation actions](/integrations/api/app/tag/automations/listCampaignActions/) endpoint.  If you pass the `multi_language_branch_action_id`, then the `metric` object shows the *total* across every variant. The `language_variants` object shows a breakdown for each language variant.   If you want to get the sum of all metrics across all of an automation's actions, you can use each action id of a multi-language message, like you would for other types of actions. 
pub async fn campaign_action_metrics(configuration: &configuration::Configuration, campaign_id: crate::ids::CampaignId, action_id: crate::ids::ActionId, version: Option<&str>, res: Option<&str>, start: Option<i64>, end: Option<i64>, tz: Option<&str>, period: Option<&str>, steps: Option<i64>, r#type: Option<&str>) -> Result<models::CampaignActionMetrics200Response, Error<CampaignActionMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_campaign_id = campaign_id;
    let p_path_action_id = action_id;
    let p_query_version = version;
    let p_query_res = res;
    let p_query_start = start;
    let p_query_end = end;
    let p_query_tz = tz;
    let p_query_period = period;
    let p_query_steps = steps;
    let p_query_type = r#type;

    let uri_str = format!("{}/v1/campaigns/{campaign_id}/actions/{action_id}/metrics", configuration.base_path, campaign_id=p_path_campaign_id, action_id=p_path_action_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_version {
        req_builder = req_builder.query(&[("version", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_res {
        req_builder = req_builder.query(&[("res", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start {
        req_builder = req_builder.query(&[("start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end {
        req_builder = req_builder.query(&[("end", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_tz {
        req_builder = req_builder.query(&[("tz", &param_value.to_string())]);
    }
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CampaignActionMetrics200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CampaignActionMetrics200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CampaignActionMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a list of Journey Metrics for your automation. These metrics show how many people triggered your automation, were messaged, etc for the time period and \"resolution\" you set. You must provide the `start`, `end`, and `resolution` parameters or your request will return `400`.  Metrics in the response are arrays, and each index in the array corresponds to the `resolution` in your request. If you request metrics in `days`, the first result in each metric array is the first day of results and each successive increment represents another day.  Each increment represents the number of journeys that started within a time period and eventually achieved a particular metric. For example, array index 0 for the `converted` metric represents the number of journeys that started on the first day/month of results that achieved a conversion. 
pub async fn campaign_journey_metrics(configuration: &configuration::Configuration, campaign_id: crate::ids::CampaignId, start: i64, end: i64, resolution: &str) -> Result<models::CampaignJourneyMetrics200Response, Error<CampaignJourneyMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_campaign_id = campaign_id;
    let p_query_start = start;
    let p_query_end = end;
    let p_query_resolution = resolution;

    let uri_str = format!("{}/v1/campaigns/{campaign_id}/journey_metrics", configuration.base_path, campaign_id=p_path_campaign_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("start", &p_query_start.to_string())]);
    req_builder = req_builder.query(&[("end", &p_query_end.to_string())]);
    req_builder = req_builder.query(&[("resolution", &p_query_resolution.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CampaignJourneyMetrics200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CampaignJourneyMetrics200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CampaignJourneyMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns metrics for link clicks within an automation, both in total and in `series` periods (days, weeks, etc). `series` metrics are ordered oldest to newest (i.e. the 0-index for any result is the oldest step/period).  You cannot request fewer than 2 steps of any period (2 hours, 2 days, 2 weeks, or 2 months). For instance, `?period=days&steps=1` means two days - the 48 hours before the API request was made. `?period=days&steps=0` returns the same as the maximum of the period - `?period=days&steps=45`. See the `steps` parameter below for the maximum count of each period. 
pub async fn campaign_link_metrics(configuration: &configuration::Configuration, campaign_id: crate::ids::CampaignId, period: Option<&str>, steps: Option<i64>, unique: Option<bool>) -> Result<models::CampaignLinkMetrics200Response, Error<CampaignLinkMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_campaign_id = campaign_id;
    let p_query_period = period;
    let p_query_steps = steps;
    let p_query_unique = unique;

    let uri_str = format!("{}/v1/campaigns/{campaign_id}/metrics/links", configuration.base_path, campaign_id=p_path_campaign_id);
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
        let entity: Option<CampaignLinkMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns metrics for an individual automation as a series of counts, oldest first—the 0-index of any result is the oldest point in the window.  **We recommend version 2. Send `version=2`.** It accepts an explicit `start`, `end`, and `res`, reported in the `tz` you choose. Without it, the endpoint answers with version 1 behavior instead: a window built from `period` and `steps`, always in Eastern time.  Version 1 can't return fewer than 2 steps of any period. `?period=days&steps=1` means two days—the 48 hours before the request—and `?period=days&steps=0` returns the maximum for the period, the same as `?period=days&steps=45`. 
pub async fn campaign_metrics(configuration: &configuration::Configuration, campaign_id: crate::ids::CampaignId, version: Option<&str>, res: Option<&str>, start: Option<i64>, end: Option<i64>, tz: Option<&str>, period: Option<&str>, steps: Option<i64>, r#type: Option<&str>) -> Result<models::CampaignMetrics200Response, Error<CampaignMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_campaign_id = campaign_id;
    let p_query_version = version;
    let p_query_res = res;
    let p_query_start = start;
    let p_query_end = end;
    let p_query_tz = tz;
    let p_query_period = period;
    let p_query_steps = steps;
    let p_query_type = r#type;

    let uri_str = format!("{}/v1/campaigns/{campaign_id}/metrics", configuration.base_path, campaign_id=p_path_campaign_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_version {
        req_builder = req_builder.query(&[("version", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_res {
        req_builder = req_builder.query(&[("res", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start {
        req_builder = req_builder.query(&[("start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end {
        req_builder = req_builder.query(&[("end", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_tz {
        req_builder = req_builder.query(&[("tz", &param_value.to_string())]);
    }
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CampaignMetrics200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CampaignMetrics200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CampaignMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns information about a specific action in an automation.
pub async fn get_campaign_action(configuration: &configuration::Configuration, campaign_id: crate::ids::CampaignId, action_id: crate::ids::ActionId) -> Result<models::GetCampaignAction200Response, Error<GetCampaignActionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_campaign_id = campaign_id;
    let p_path_action_id = action_id;

    let uri_str = format!("{}/v1/campaigns/{campaign_id}/actions/{action_id}", configuration.base_path, campaign_id=p_path_campaign_id, action_id=p_path_action_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCampaignAction200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCampaignAction200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCampaignActionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a translated version of a message in an automation. The message is identified by the `action_id`.
pub async fn get_campaign_action_translation(configuration: &configuration::Configuration, campaign_id: crate::ids::CampaignId, action_id: crate::ids::ActionId, language: &str) -> Result<models::GetCampaignAction200Response, Error<GetCampaignActionTranslationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_campaign_id = campaign_id;
    let p_path_action_id = action_id;
    let p_path_language = language;

    let uri_str = format!("{}/v1/campaigns/{campaign_id}/actions/{action_id}/language/{language}", configuration.base_path, campaign_id=p_path_campaign_id, action_id=p_path_action_id, language=crate::app::apis::urlencode(p_path_language));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCampaignAction200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCampaignAction200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCampaignActionTranslationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns information about the deliveries (instances of messages sent to individual people) sent from an automation. Provide query parameters to refine the metrics you want to return. Use the `start_ts` and `end_ts` to find messages within a time range. If your request doesn't include `start_ts` and `end_ts` parameters, we'll return the most recent 6 months of messages. If your `start_ts` and `end_ts` range is more than 12 months, we'll return 12 months of data from the most recent timestamp in your request. Timestamps reflect when deliveries were created in our system, not when they were actually sent to recipients. There may be a delay between creation and sending.
pub async fn get_campaign_messages(configuration: &configuration::Configuration, campaign_id: crate::ids::CampaignId, start: Option<&str>, limit: Option<i64>, r#type: Option<&str>, metric: Option<&str>, drafts: Option<bool>, start_ts: Option<i64>, end_ts: Option<i64>, get_tracked_responses: Option<bool>) -> Result<models::GetCampaignMessages200Response, Error<GetCampaignMessagesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_campaign_id = campaign_id;
    let p_query_start = start;
    let p_query_limit = limit;
    let p_query_type = r#type;
    let p_query_metric = metric;
    let p_query_drafts = drafts;
    let p_query_start_ts = start_ts;
    let p_query_end_ts = end_ts;
    let p_query_get_tracked_responses = get_tracked_responses;

    let uri_str = format!("{}/v1/campaigns/{campaign_id}/messages", configuration.base_path, campaign_id=p_path_campaign_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_start {
        req_builder = req_builder.query(&[("start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_metric {
        req_builder = req_builder.query(&[("metric", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_drafts {
        req_builder = req_builder.query(&[("drafts", &param_value.to_string())]);
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
        let entity: Option<GetCampaignMessagesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns metadata for an individual automation.
pub async fn get_campaigns(configuration: &configuration::Configuration, campaign_id: crate::ids::CampaignId) -> Result<models::GetCampaigns200Response, Error<GetCampaignsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_campaign_id = campaign_id;

    let uri_str = format!("{}/v1/campaigns/{campaign_id}", configuration.base_path, campaign_id=p_path_campaign_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCampaigns200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCampaigns200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCampaignsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the operations in an automation workflow. Each object in the response represents an action or 'tile' in the automation builder.  This endpoint returns up to 10 `actions` at a time. If there is another page of results, the response will include a `next` string. Pass this string as the `start` parameter to get the next page of results.  You get the actions that have content, like messages, attribute updates, and events—not delays or flow control blocks like t/f branches.  If a message block contains multiple languages, this endpoint returns an object for each language variant. Each language variant includes the `multi_language_branch_action_id` property with the same value. Use this id to retrieve metrics for each language variant in a single request through [Get automation action metrics](/integrations/api/app/tag/automations/campaignActionMetrics/). 
pub async fn list_campaign_actions(configuration: &configuration::Configuration, campaign_id: crate::ids::CampaignId, start: Option<&str>) -> Result<models::ListCampaignActions200Response, Error<ListCampaignActionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_campaign_id = campaign_id;
    let p_query_start = start;

    let uri_str = format!("{}/v1/campaigns/{campaign_id}/actions", configuration.base_path, campaign_id=p_path_campaign_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_start {
        req_builder = req_builder.query(&[("start", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListCampaignActions200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListCampaignActions200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListCampaignActionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a list of your automations and associated metadata.
pub async fn list_campaigns(configuration: &configuration::Configuration, ) -> Result<models::ListCampaigns200Response, Error<ListCampaignsError>> {

    let uri_str = format!("{}/v1/campaigns", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListCampaigns200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListCampaigns200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListCampaignsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update the contents of an automation action, including the body of messages and HTTP requests.  **NOTE**: You can't manage content made with Design Studio with this endpoint. Use the [Design Studio APIs](/integrations/api/design-studio/) instead. 
pub async fn update_campaign_action(configuration: &configuration::Configuration, campaign_id: crate::ids::CampaignId, action_id: crate::ids::ActionId, update_action_object: Option<models::UpdateActionObject>) -> Result<models::GetCampaignAction200Response, Error<UpdateCampaignActionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_campaign_id = campaign_id;
    let p_path_action_id = action_id;
    let p_body_update_action_object = update_action_object;

    let uri_str = format!("{}/v1/campaigns/{campaign_id}/actions/{action_id}", configuration.base_path, campaign_id=p_path_campaign_id, action_id=p_path_action_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_action_object);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCampaignAction200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCampaignAction200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateCampaignActionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update the contents of a language variant of an automation action, including the body of the messages and HTTP requests.  **NOTE**: You can't manage content made with Design Studio with this endpoint. Use the [Design Studio APIs](/integrations/api/design-studio/) instead. 
pub async fn update_campaign_action_translation(configuration: &configuration::Configuration, campaign_id: crate::ids::CampaignId, action_id: crate::ids::ActionId, language: &str, update_action_object: Option<models::UpdateActionObject>) -> Result<models::GetCampaignAction200Response, Error<UpdateCampaignActionTranslationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_campaign_id = campaign_id;
    let p_path_action_id = action_id;
    let p_path_language = language;
    let p_body_update_action_object = update_action_object;

    let uri_str = format!("{}/v1/campaigns/{campaign_id}/actions/{action_id}/language/{language}", configuration.base_path, campaign_id=p_path_campaign_id, action_id=p_path_action_id, language=crate::app::apis::urlencode(p_path_language));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_action_object);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCampaignAction200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCampaignAction200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateCampaignActionTranslationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

