/*
 * Customer.io App API
 *
 * Our App API provides ways to trigger messages and retrieve information about people, automations, broadcasts, and more.  # Overview  The App API provides methods to send newsletters, transactional messages, and API-triggered broadcasts. You can create newsletters from scratch and update transactional messages and API-triggered broadcasts.  For transactional messages and API-triggered broadcasts, your payload acts as a message \"trigger\" and can contain `data` that you reference in your messages using liquid—`{{trigger.<data>}}`.  The other endpoints help you retrieve information about people, segments, automations, broadcasts, etc; it also lets you update automation actions, messages, newsletter variants, etc. Aside from the [API-triggered broadcast](/integrations/api/app/tag/send-messages/triggerBroadcast/) endpoint (1 per 10 seconds), requests are limited to 10 per second. [Transactional](/integrations/api/app/tag/send-messages/sendEmail/) messages go through the same high-throughput ingress as our Track and Pipelines APIs and share their soft rate limit of 3000 requests per 3 seconds, which isn't strictly enforced.  # We may add to these APIs  As we release new features, we add endpoints and we may even add fields to existing endpoints. We don't announce these additions in advance. Fields we add will be backward compatible: existing fields keep their names and types.  Build your integration to ignore fields it doesn't recognize. If you validate responses against a schema, make sure the schema accepts unknown keys. In JSON Schema, for example, don't set `additionalProperties: false`.  # Use our Postman collection  We've generated a Postman collection to help you get started with our APIs.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  **NOTE**: Postman endpoints default to our US APIs. If you're in our European (EU) region, you'll need to add `-eu` to the server variables (`track_api_url` and `app_api_url`).  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d%26entityType%3Dcollection%26workspaceId%3Db886877f-fc09-475f-84fe-6221a98f4d18#?env%5BCustomer.io%20API%20Environment%5D=W3sia2V5IjoidHJhY2tfYXBpX3VybCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiJ0cmFjay5jdXN0b21lci5pbyIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiYXBpLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzaXRlX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBpX2tleSIsInR5cGUiOiJzZWNyZXQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYmVhcmVyIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYnJvYWRjYXN0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaW1wb3J0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiZW1haWxfYWRkcmVzcyIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN1cHByZXNzaW9uX3R5cGUiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjb2xsZWN0aW9uX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoic25pcHBldF9uYW1lIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoid2ViaG9va19pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InNlbmRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImV4cG9ydF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6Im1lc3NhZ2VfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzZWdtZW50X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoibmV3c2xldHRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImNvbnRlbnRfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjYW1wYWlnbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImFjdGlvbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImN1c3RvbWVyX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoidHJhbnNhY3Rpb25hbF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRyaWdnZXJfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJmb3JtX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaWRlbnRpZmllciIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRldmljZV9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRlbGl2ZXJ5X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9XQ==)  # Server addresses: US and EU  Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://api.customer.io | | EU | https://api-eu.customer.io |  # Authentication  All requests to the Customer.io App API use an [App API Key](#authentication).  To authenticate, provide your key as a Bearer token in a HTTP Authorization header. You can create and manage your API keys—including keys with different scopes—in [your account settings page](https://fly.customer.io/settings/api_credentials?keyType=app). Each operation on this page references the authorization header it requires.  # Rate Limits  Most endpoints on this page are limited to 10 requests per second. The exceptions are: * [Transactional messages](/integrations/api/app/tag/send-messages/sendEmail/) use the same soft rate limit of our Track and Pipelines APIs (3000 requests per 3 seconds, not strictly enforced). * The [API-triggered broadcast endpoint](/integrations/api/app/tag/send-messages/triggerBroadcast/) is limited to 1 request every 10 seconds.  **Rate limits are subject to change. We may adjust these thresholds to ensure stable performance for all customers.** 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::app::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`create_sms`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSmsError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status422(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_sms`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSmsError {
    Status401(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sms`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSmsError {
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`link_sms`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkSmsError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    Status422(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_sms`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSmsError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`publish_sms`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublishSmsError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status422(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_sms`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSmsError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status422(models::Error),
    UnknownValue(serde_json::Value),
}


/// This endpoint creates an SMS message. After you create an SMS, [link it](/integrations/api/app/tag/design-studio-sms/linkSms/) to a workflow and [publish it](/integrations/api/app/tag/design-studio-sms/publishSms/) to send it to your audience. 
pub async fn create_sms(configuration: &configuration::Configuration, create_sms_request: models::CreateSmsRequest) -> Result<models::CreateSms200Response, Error<CreateSmsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_sms_request = create_sms_request;

    let uri_str = format!("{}/v1/design_studio/sms", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_sms_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateSms200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateSms200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateSmsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Delete an SMS. You can't delete an SMS that's linked to a workflow, like a transactional message, a one-time send, or an automation action. 
pub async fn delete_sms(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<DeleteSmsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/sms/{id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteSmsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a single SMS, including its content, recipient, and sender. 
pub async fn get_sms(configuration: &configuration::Configuration, id: &str) -> Result<models::CreateSms200Response, Error<GetSmsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/sms/{id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateSms200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateSms200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSmsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Link an SMS to a workflow or action: a transactional message, a one-time send, an automation action, or an API-triggered broadcast.  Linking copies the SMS's current content, recipient, and sender to the workflow right away. If the workflow is already live, people start receiving this SMS as soon as you link it. After that, [publish the SMS](/integrations/api/app/tag/design-studio-sms/publishSms/) whenever you want to push later edits to the workflow.  You can only link an SMS to one workflow or action at a time. If you try to link an SMS to a workflow or action that's already linked to a *different* message, the request fails with a `409` unless you pass `\"force\": true`. The `force` parameter replaces (and unlinks) the other message.  Create the SMS first, either in Design Studio or with [the API](/integrations/api/app/tag/design-studio-sms/createSms/). 
pub async fn link_sms(configuration: &configuration::Configuration, id: &str, link_request: models::LinkRequest) -> Result<models::LinkResponse, Error<LinkSmsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_link_request = link_request;

    let uri_str = format!("{}/v1/design_studio/sms/{id}/link", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_link_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::LinkResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::LinkResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<LinkSmsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a paginated list of your SMS messages and a separate array of the folders they belong to. Each item in the list is a summary; to see an SMS's content, recipient, and sender, [get the SMS](/integrations/api/app/tag/design-studio-sms/getSms/). 
pub async fn list_sms(configuration: &configuration::Configuration, page: Option<i64>, limit: Option<i64>, parent_folder_id: Option<&str>, direct_descendants_only: Option<bool>, sort_by: Option<&str>, sort_order: Option<&str>, created_before: Option<i64>, created_after: Option<i64>, updated_before: Option<i64>, updated_after: Option<i64>) -> Result<models::ListSms200Response, Error<ListSmsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_page = page;
    let p_query_limit = limit;
    let p_query_parent_folder_id = parent_folder_id;
    let p_query_direct_descendants_only = direct_descendants_only;
    let p_query_sort_by = sort_by;
    let p_query_sort_order = sort_order;
    let p_query_created_before = created_before;
    let p_query_created_after = created_after;
    let p_query_updated_before = updated_before;
    let p_query_updated_after = updated_after;

    let uri_str = format!("{}/v1/design_studio/sms", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_parent_folder_id {
        req_builder = req_builder.query(&[("parent_folder_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_direct_descendants_only {
        req_builder = req_builder.query(&[("direct_descendants_only", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_sort_by {
        req_builder = req_builder.query(&[("sort_by", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_sort_order {
        req_builder = req_builder.query(&[("sort_order", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_created_before {
        req_builder = req_builder.query(&[("created_before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_created_after {
        req_builder = req_builder.query(&[("created_after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_updated_before {
        req_builder = req_builder.query(&[("updated_before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_updated_after {
        req_builder = req_builder.query(&[("updated_after", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListSms200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListSms200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListSmsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Publish a linked SMS, pushing its current content live to the workflow it's linked to. You need to link the SMS and set a sender before you can publish it. 
pub async fn publish_sms(configuration: &configuration::Configuration, id: &str) -> Result<models::ChannelPublishResponse, Error<PublishSmsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/sms/{id}/publish", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ChannelPublishResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ChannelPublishResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PublishSmsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update an SMS's name, folder, content, recipient, or sender. You only need to send the fields you want to change; Customer.io keeps the current value of any field you leave out.  Updating an SMS doesn't change what your audience receives. If the SMS is linked to a workflow, you need to [publish it](/integrations/api/app/tag/design-studio-sms/publishSms/) to push your changes live. 
pub async fn update_sms(configuration: &configuration::Configuration, id: &str, update_sms_request: models::UpdateSmsRequest) -> Result<(), Error<UpdateSmsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_update_sms_request = update_sms_request;

    let uri_str = format!("{}/v1/design_studio/sms/{id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_sms_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateSmsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

