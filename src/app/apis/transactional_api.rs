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


/// struct for typed errors of method [`get_transactional`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransactionalError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_transactional_translation_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransactionalTranslationByIdError {
    Status400(models::SendEmail400Response),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_transactional_variant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransactionalVariantError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_transactional`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTransactionalError {
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_transactional_variants`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTransactionalVariantsError {
    Status400(models::SendEmail400Response),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transactional_links`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionalLinksError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transactional_messages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionalMessagesError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transactional_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionalMetricsError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_transactional`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTransactionalError {
    Status400(models::SendEmail400Response),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_transactional_variant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTransactionalVariantError {
    Status400(models::SendEmail400Response),
    UnknownValue(serde_json::Value),
}


/// Returns information about an individual transactional message.
pub async fn get_transactional(configuration: &configuration::Configuration, transactional_id: crate::ids::TransactionalMessageId) -> Result<models::GetTransactional200Response, Error<GetTransactionalError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_transactional_id = transactional_id;

    let uri_str = format!("{}/v1/transactional/{transactional_id}", configuration.base_path, transactional_id=p_path_transactional_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetTransactional200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetTransactional200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTransactionalError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns an individual translation variant of a transactional message, including the message content. To look up a translation by its language instead of its id, see [Get a translation of a transactional message](/integrations/api/app/tag/transactional/getTransactionalVariant/).
pub async fn get_transactional_translation_by_id(configuration: &configuration::Configuration, transactional_id: crate::ids::TransactionalMessageId, content_id: crate::ids::ContentId) -> Result<models::GetTransactionalTranslationById200Response, Error<GetTransactionalTranslationByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_transactional_id = transactional_id;
    let p_path_content_id = content_id;

    let uri_str = format!("{}/v1/transactional/{transactional_id}/contents/{content_id}", configuration.base_path, transactional_id=p_path_transactional_id, content_id=p_path_content_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetTransactionalTranslationById200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetTransactionalTranslationById200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTransactionalTranslationByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns information about a translation of an individual transactional message, including the message content. To look up a translation by its id instead of its language, see [Get a translation of a transactional by ID](/integrations/api/app/tag/transactional/getTransactionalTranslationById/).
pub async fn get_transactional_variant(configuration: &configuration::Configuration, transactional_id: crate::ids::TransactionalMessageId, language: &str) -> Result<models::UpdateTransactional200Response, Error<GetTransactionalVariantError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_transactional_id = transactional_id;
    let p_path_language = language;

    let uri_str = format!("{}/v1/transactional/{transactional_id}/language/{language}", configuration.base_path, transactional_id=p_path_transactional_id, language=crate::app::apis::urlencode(p_path_language));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UpdateTransactional200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UpdateTransactional200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTransactionalVariantError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a list of your transactional messages—the transactional IDs that you use to trigger an individual transactional delivery. This endpoint does not return information about deliveries (instances of a message sent to a person) themselves.
pub async fn list_transactional(configuration: &configuration::Configuration, ) -> Result<models::ListTransactional200Response, Error<ListTransactionalError>> {

    let uri_str = format!("{}/v1/transactional", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListTransactional200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListTransactional200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListTransactionalError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the content variants of a transactional message, where each variant represents a different language.
pub async fn list_transactional_variants(configuration: &configuration::Configuration, transactional_id: crate::ids::TransactionalMessageId) -> Result<models::ListTransactionalVariants200Response, Error<ListTransactionalVariantsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_transactional_id = transactional_id;

    let uri_str = format!("{}/v1/transactional/{transactional_id}/contents", configuration.base_path, transactional_id=p_path_transactional_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListTransactionalVariants200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListTransactionalVariants200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListTransactionalVariantsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns metrics for clicked links from a transactional message, both in total and in `series` periods (days, weeks, etc). `series` metrics are ordered oldest to newest (i.e. the 0-index for any result is the oldest step/period).  You cannot request fewer than 2 steps of any period (2 hours, 2 days, 2 weeks, or 2 months). For instance, `?period=days&steps=1` means two days - the 48 hours before the API request was made. `?period=days&steps=0` returns the same as the maximum of the period - `?period=days&steps=45`. See the `steps` parameter below for the maximum count of each period. 
pub async fn transactional_links(configuration: &configuration::Configuration, transactional_id: crate::ids::TransactionalMessageId, period: Option<&str>, steps: Option<i64>, unique: Option<bool>) -> Result<models::CampaignLinkMetrics200Response, Error<TransactionalLinksError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_transactional_id = transactional_id;
    let p_query_period = period;
    let p_query_steps = steps;
    let p_query_unique = unique;

    let uri_str = format!("{}/v1/transactional/{transactional_id}/metrics/links", configuration.base_path, transactional_id=p_path_transactional_id);
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
        let entity: Option<TransactionalLinksError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns information about the deliveries (instances of messages sent to individual people) from a transactional message. Provide query parameters to refine the metrics you want to return.  Use the `start_ts` and `end_ts` to find messages within a time range. If your request doesn't include `start_ts` and `end_ts` parameters, we'll return the most recent 6 months of messages. If your `start_ts` and `end_ts` range is more than 12 months, we'll return 12 months of data from the most recent timestamp in your request. Timestamps reflect when deliveries were created in our system, not when they were actually sent to recipients. There may be a delay between creation and sending. 
pub async fn transactional_messages(configuration: &configuration::Configuration, transactional_id: crate::ids::TransactionalMessageId, start: Option<&str>, limit: Option<i64>, metric: Option<&str>, state: Option<&str>, start_ts: Option<i64>, end_ts: Option<i64>, get_tracked_responses: Option<bool>) -> Result<models::GetCampaignMessages200Response, Error<TransactionalMessagesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_transactional_id = transactional_id;
    let p_query_start = start;
    let p_query_limit = limit;
    let p_query_metric = metric;
    let p_query_state = state;
    let p_query_start_ts = start_ts;
    let p_query_end_ts = end_ts;
    let p_query_get_tracked_responses = get_tracked_responses;

    let uri_str = format!("{}/v1/transactional/{transactional_id}/messages", configuration.base_path, transactional_id=p_path_transactional_id);
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
        let entity: Option<TransactionalMessagesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a list of metrics for a transactional message in `steps` (days, weeks, etc). We return metrics from oldest to newest (i.e. the 0-index for any result is the oldest step/period).  You cannot request fewer than 2 steps of any period (2 hours, 2 days, 2 weeks, or 2 months). For instance, `?period=days&steps=1` means two days - the 48 hours before the API request was made. `?period=days&steps=0` returns the same as the maximum of the period - `?period=days&steps=45`. See the `steps` parameter below for the maximum count of each period. 
pub async fn transactional_metrics(configuration: &configuration::Configuration, transactional_id: crate::ids::TransactionalMessageId, period: Option<&str>, steps: Option<i64>) -> Result<models::BroadcastMetrics200Response, Error<TransactionalMetricsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_transactional_id = transactional_id;
    let p_query_period = period;
    let p_query_steps = steps;

    let uri_str = format!("{}/v1/transactional/{transactional_id}/metrics", configuration.base_path, transactional_id=p_path_transactional_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_period {
        req_builder = req_builder.query(&[("period", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_steps {
        req_builder = req_builder.query(&[("steps", &param_value.to_string())]);
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
        let entity: Option<TransactionalMetricsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update the body of a transactional email. This fully overwrites your existing transactional message. We'll use your updated content for any future transactional requests (`/v1/send/email`), so make sure that you test your message before you update it.   **NOTE**: You cannot manage content made with Design Studio with this endpoint. Use the [Design Studio APIs](/integrations/api/design-studio/) instead. 
pub async fn update_transactional(configuration: &configuration::Configuration, transactional_id: crate::ids::TransactionalMessageId, content_id: crate::ids::ContentId, update_transactional_action_object: Option<models::UpdateTransactionalActionObject>) -> Result<models::UpdateTransactional200Response, Error<UpdateTransactionalError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_transactional_id = transactional_id;
    let p_path_content_id = content_id;
    let p_body_update_transactional_action_object = update_transactional_action_object;

    let uri_str = format!("{}/v1/transactional/{transactional_id}/content/{content_id}", configuration.base_path, transactional_id=p_path_transactional_id, content_id=p_path_content_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_transactional_action_object);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UpdateTransactional200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UpdateTransactional200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateTransactionalError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update the body and other data of a specific language variant for a transactional message. This fully overwrites this specific translation of your existing transactional message.   **NOTE**: You cannot manage content made with Design Studio with this endpoint. Use the [Design Studio APIs](/integrations/api/design-studio/) instead. 
pub async fn update_transactional_variant(configuration: &configuration::Configuration, transactional_id: crate::ids::TransactionalMessageId, language: &str, update_transactional_action_object: Option<models::UpdateTransactionalActionObject>) -> Result<models::UpdateTransactional200Response, Error<UpdateTransactionalVariantError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_transactional_id = transactional_id;
    let p_path_language = language;
    let p_body_update_transactional_action_object = update_transactional_action_object;

    let uri_str = format!("{}/v1/transactional/{transactional_id}/language/{language}", configuration.base_path, transactional_id=p_path_transactional_id, language=crate::app::apis::urlencode(p_path_language));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_transactional_action_object);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UpdateTransactional200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UpdateTransactional200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateTransactionalVariantError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

