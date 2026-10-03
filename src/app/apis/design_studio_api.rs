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


/// struct for typed errors of method [`create_component`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateComponentError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_folder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFolderError {
    Status400(models::CreateFolder400Response),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_component`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteComponentError {
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_folder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFolderError {
    Status401(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_component`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetComponentError {
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_folder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFolderError {
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_components`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListComponentsError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_folders`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFoldersError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_component`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateComponentError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_folder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFolderError {
    Status400(models::CreateFolder400Response),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}


/// Creates a custom component.
pub async fn create_component(configuration: &configuration::Configuration, create_component_request: models::CreateComponentRequest) -> Result<models::CreateComponent200Response, Error<CreateComponentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_component_request = create_component_request;

    let uri_str = format!("{}/v1/design_studio/components", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_component_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateComponent200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateComponent200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateComponentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Create a new folder at the root level or under a parent folder. To create a child folder, you need the UUID of the parent folder, which you can retrieve with [List folders](/integrations/api/app/tag/design-studio/listFolders/). 
pub async fn create_folder(configuration: &configuration::Configuration, create_folder_request: models::CreateFolderRequest) -> Result<models::CreateFolder200Response, Error<CreateFolderError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_folder_request = create_folder_request;

    let uri_str = format!("{}/v1/design_studio/folders", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_folder_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateFolder200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateFolder200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateFolderError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Delete a component. Note, this deletes any component. If you delete a component in use, the emails that reference it could fail to send.
pub async fn delete_component(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<DeleteComponentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/components/{id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
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
        let entity: Option<DeleteComponentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Delete a folder **including subfolders and all file (components, templates, and emails)**. You cannot delete a folder with emails used in your workflows (automations, broadcasts, etc).  However, you can delete a folder with components that are referenced in emails linked to workflows, so make sure deleting a folder with components won't break your emails. 
pub async fn delete_folder(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<DeleteFolderError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/folders/{id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
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
        let entity: Option<DeleteFolderError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a single component with its full content.
pub async fn get_component(configuration: &configuration::Configuration, id: &str) -> Result<models::CreateComponent200Response, Error<GetComponentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/components/{id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateComponent200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateComponent200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetComponentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get a folder by its UUID. You can retrieve the UUID of folders through [List folders](/integrations/api/app/tag/design-studio/listFolders/). 
pub async fn get_folder(configuration: &configuration::Configuration, id: &str) -> Result<models::CreateFolder200Response, Error<GetFolderError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/folders/{id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateFolder200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateFolder200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFolderError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a paginated list of components and any folders in the result set.
pub async fn list_components(configuration: &configuration::Configuration, tag: Option<&str>, page: Option<i64>, limit: Option<i64>, parent_folder_id: Option<&str>, direct_descendants_only: Option<bool>, sort_by: Option<&str>, sort_order: Option<&str>, created_before: Option<i64>, created_after: Option<i64>, updated_before: Option<i64>, updated_after: Option<i64>) -> Result<models::ListComponents200Response, Error<ListComponentsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_tag = tag;
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

    let uri_str = format!("{}/v1/design_studio/components", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_tag {
        req_builder = req_builder.query(&[("tag", &param_value.to_string())]);
    }
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListComponents200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListComponents200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListComponentsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a paginated list of folders. This does not include files like emails, components, etc. 
pub async fn list_folders(configuration: &configuration::Configuration, page: Option<i64>, limit: Option<i64>, parent_folder_id: Option<&str>, direct_descendants_only: Option<bool>, sort_by: Option<&str>, sort_order: Option<&str>, created_before: Option<i64>, created_after: Option<i64>, updated_before: Option<i64>, updated_after: Option<i64>) -> Result<models::ListFolders200Response, Error<ListFoldersError>> {
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

    let uri_str = format!("{}/v1/design_studio/folders", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListFolders200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListFolders200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListFoldersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update part of a component: its name, tag, folder, or content. 
pub async fn update_component(configuration: &configuration::Configuration, id: &str, update_component_request: models::UpdateComponentRequest) -> Result<(), Error<UpdateComponentError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_update_component_request = update_component_request;

    let uri_str = format!("{}/v1/design_studio/components/{id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_component_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateComponentError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update part of a folder: the name and/or the folder it belongs to. If you move a folder, all files stay nested in the folder. 
pub async fn update_folder(configuration: &configuration::Configuration, id: &str, update_folder_request: models::UpdateFolderRequest) -> Result<(), Error<UpdateFolderError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_update_folder_request = update_folder_request;

    let uri_str = format!("{}/v1/design_studio/folders/{id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_folder_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateFolderError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

