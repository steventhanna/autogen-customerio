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


/// struct for typed errors of method [`check_unpublished_changes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckUnpublishedChangesError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEmailError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_email_translation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEmailTranslationError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteEmailError {
    Status401(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_email_translation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteEmailTranslationError {
    Status401(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteVersionError {
    Status401(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEmailError {
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_email_translation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEmailTranslationError {
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_inbox_preview_capture`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInboxPreviewCaptureError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_inbox_preview_credits`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInboxPreviewCreditsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_inbox_preview_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInboxPreviewJobError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_publish_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPublishStatusError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    Status422(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVersionError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`link_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkEmailError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_email_translations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEmailTranslationsError {
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_emails`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEmailsError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_inbox_preview_clients`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListInboxPreviewClientsError {
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    Status503(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_inbox_preview_jobs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListInboxPreviewJobsError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status429(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_versions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVersionsError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`preview_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PreviewEmailError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`publish_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublishEmailError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    Status422(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`render_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RenderEmailError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`restore_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RestoreVersionError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`review_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReviewEmailError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`save_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SaveVersionError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`submit_inbox_preview`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitInboxPreviewError {
    Status400(models::Error),
    Status401(models::Error),
    Status403(models::Error),
    Status404(models::Error),
    Status409(models::Error),
    Status422(models::Error),
    Status429(models::Error),
    Status503(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`test_send_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestSendEmailError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    Status422(models::Error),
    Status429(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateEmailError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_email_translation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateEmailTranslationError {
    Status400(models::Error),
    Status401(models::Error),
    Status404(models::Error),
    UnknownValue(serde_json::Value),
}


/// Check whether a linked email, or any of its translations, has content changes that haven't been published yet. Use this to decide whether you need to publish before changes go live.  Returns `false` for emails that aren't linked to a workflow or that have no pending changes. 
pub async fn check_unpublished_changes(configuration: &configuration::Configuration, id: &str) -> Result<models::UnpublishedChangesResponse, Error<CheckUnpublishedChangesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/unpublished_changes", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UnpublishedChangesResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UnpublishedChangesResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CheckUnpublishedChangesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Create an email. Note, you can create an email without filling out all required fields for sending. You can fill in the envelope, like a to and from address, with this method, but that's not required until you link it to a workflow like an automation.
pub async fn create_email(configuration: &configuration::Configuration, email_create: models::EmailCreate) -> Result<models::CreateEmail200Response, Error<CreateEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_email_create = email_create;

    let uri_str = format!("{}/v1/design_studio/emails", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_email_create);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateEmail200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateEmail200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Creates a new translation for an email. If content, envelope, and/or transformers are omitted, the values are copied from the default (parent) email. 
pub async fn create_email_translation(configuration: &configuration::Configuration, id: &str, create_email_translation_request: models::CreateEmailTranslationRequest) -> Result<models::CreateEmailTranslation200Response, Error<CreateEmailTranslationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_create_email_translation_request = create_email_translation_request;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/languages", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_email_translation_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateEmailTranslation200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateEmailTranslation200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateEmailTranslationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Delete an email. You cannot delete an email that is linked to a workflow (automation, broadcast, etc). This deletes the email and all translations. 
pub async fn delete_email(configuration: &configuration::Configuration, id: &str) -> Result<(), Error<DeleteEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/emails/{id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
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
        let entity: Option<DeleteEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Delete a specific language translation from an email. This fails if the email is linked to a workflow (automation, broadcast, etc). 
pub async fn delete_email_translation(configuration: &configuration::Configuration, id: &str, language: &str) -> Result<(), Error<DeleteEmailTranslationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_language = language;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/languages/{language}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id), language=crate::app::apis::urlencode(p_path_language));
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
        let entity: Option<DeleteEmailTranslationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Deletes a version. You can't delete a version that Design Studio created automatically during a publish. 
pub async fn delete_version(configuration: &configuration::Configuration, id: &str, version_id: &str) -> Result<(), Error<DeleteVersionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_version_id = version_id;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/versions/{version_id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id), version_id=crate::app::apis::urlencode(p_path_version_id));
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
        let entity: Option<DeleteVersionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a single email including content, envelope details, and transformers. This endpoint returns only default emails; see [Email translations](/integrations/api/app/tag/design-studio-emails/createEmailTranslation/) to access language variants. 
pub async fn get_email(configuration: &configuration::Configuration, id: &str) -> Result<models::CreateEmail200Response, Error<GetEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/emails/{id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateEmail200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateEmail200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a single email translation by language code, including content, envelope, and transformers. 
pub async fn get_email_translation(configuration: &configuration::Configuration, id: &str, language: &str) -> Result<models::CreateEmailTranslation200Response, Error<GetEmailTranslationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_language = language;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/languages/{language}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id), language=crate::app::apis::urlencode(p_path_language));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateEmailTranslation200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateEmailTranslation200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetEmailTranslationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the image bytes for a single preview job, proxied from the vendor. The response's content type matches whatever the vendor sent for that capture, which isn't always PNG, and the `Content-Disposition` filename's extension matches it. You can also use the URL paths in the response of [Get an inbox preview job](/integrations/api/app/tag/design-studio-emails/getInboxPreviewJob/) to complete this request. 
pub async fn get_inbox_preview_capture(configuration: &configuration::Configuration, id: &str, run_id: i64, client_id: &str, variant: Option<&str>, key: Option<&str>) -> Result<reqwest::Response, Error<GetInboxPreviewCaptureError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_run_id = run_id;
    let p_path_client_id = client_id;
    let p_query_variant = variant;
    let p_query_key = key;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/inbox_previews/{run_id}/captures/{client_id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id), run_id=p_path_run_id, client_id=crate::app::apis::urlencode(p_path_client_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_variant {
        req_builder = req_builder.query(&[("variant", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_key {
        req_builder = req_builder.query(&[("key", &param_value.to_string())]);
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

    if !status.is_client_error() && !status.is_server_error() {
        Ok(resp)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetInboxPreviewCaptureError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns your account's credit balance for inbox previews. Your credit balance determines how many previews you can generate in your account.  Learn more about [how inbox previews use credits](/accounts/billing/inbox-previews/). 
pub async fn get_inbox_preview_credits(configuration: &configuration::Configuration, ) -> Result<models::GetInboxPreviewCredits200Response, Error<GetInboxPreviewCreditsError>> {

    let uri_str = format!("{}/v1/design_studio/inbox_previews/credits", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetInboxPreviewCredits200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetInboxPreviewCredits200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetInboxPreviewCreditsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the status of a preview job submitted with [Send for inbox previews](/integrations/api/app/tag/design-studio-emails/submitInboxPreview/), including each preview's settings. 
pub async fn get_inbox_preview_job(configuration: &configuration::Configuration, id: &str, run_id: i64) -> Result<models::InboxPreviewJob, Error<GetInboxPreviewJobError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_run_id = run_id;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/inbox_previews/{run_id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id), run_id=p_path_run_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::InboxPreviewJob`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::InboxPreviewJob`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetInboxPreviewJobError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Check the status of a multi-language publish that returned `\"status\": \"pending\"`. Pass the `publish_id` from the [Publish an email](/integrations/api/app/tag/design-studio-emails/publishEmail/) response. When the publish is still running, the response returns `\"status\": \"pending\"`; when it finishes, it returns `\"status\": \"done\"` with the template version mappings.  If the publish fails, this endpoint returns an error rather than a status. You get the same error a single-language publish returns inline—a `422` for liquid errors, a `409` when the destination automation is backfilling, and so on. 
pub async fn get_publish_status(configuration: &configuration::Configuration, id: &str, publish_id: &str) -> Result<models::PublishStatusResponse, Error<GetPublishStatusError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_query_publish_id = publish_id;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/publish_status", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("publish_id", &p_query_publish_id.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PublishStatusResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PublishStatusResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPublishStatusError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a snapshot of the version's content including the code for any custom components referenced in the email (see `dependencies` for more). 
pub async fn get_version(configuration: &configuration::Configuration, id: &str, version_id: &str) -> Result<models::GetVersion200Response, Error<GetVersionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_version_id = version_id;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/versions/{version_id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id), version_id=crate::app::apis::urlencode(p_path_version_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetVersion200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetVersion200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetVersionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Link a Design Studio email to a workflow: a transactional message, a one-time send, an automation, or an API-triggered broadcast. After you link an email, [publish it](/integrations/api/app/tag/design-studio/publishEmail/) to push the content live.  You can only link an email to one workflow or action at a time. If the workflow or action is already linked to a *different* Design Studio email, the request fails with a `409` unless you pass `\"force\": true`, which replaces (and unlinks) the other email. 
pub async fn link_email(configuration: &configuration::Configuration, id: &str, link_request: models::LinkRequest) -> Result<models::LinkResponse, Error<LinkEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_link_request = link_request;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/link", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
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
        let entity: Option<LinkEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns all translations for an email. Each translation contains the email's content, envelope, and transformers for a specific language. 
pub async fn list_email_translations(configuration: &configuration::Configuration, id: &str) -> Result<models::ListEmailTranslations200Response, Error<ListEmailTranslationsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/languages", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListEmailTranslations200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListEmailTranslations200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListEmailTranslationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a paginated list of emails and a separate array of folders that the emails belong to. 
pub async fn list_emails(configuration: &configuration::Configuration, page: Option<i64>, limit: Option<i64>, parent_folder_id: Option<&str>, direct_descendants_only: Option<bool>, sort_by: Option<&str>, sort_order: Option<&str>, created_before: Option<i64>, created_after: Option<i64>, updated_before: Option<i64>, updated_after: Option<i64>, is_template: Option<&str>, has_translations: Option<&str>, is_linked: Option<&str>) -> Result<models::ListEmails200Response, Error<ListEmailsError>> {
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
    let p_query_is_template = is_template;
    let p_query_has_translations = has_translations;
    let p_query_is_linked = is_linked;

    let uri_str = format!("{}/v1/design_studio/emails", configuration.base_path);
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
    if let Some(ref param_value) = p_query_is_template {
        req_builder = req_builder.query(&[("is_template", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_has_translations {
        req_builder = req_builder.query(&[("has_translations", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_is_linked {
        req_builder = req_builder.query(&[("is_linked", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListEmails200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListEmails200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListEmailsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the catalog of email clients and devices available for [inbox previews](/messaging/design-studio/emails/qa-in-design-studio/#inbox-preview).   Use the `id` values returned by this endpoint as `client_ids` when you [send for an inbox preview](/integrations/api/app/tag/design-studio-emails/submitInboxPreview/). 
pub async fn list_inbox_preview_clients(configuration: &configuration::Configuration, ) -> Result<models::ListInboxPreviewClients200Response, Error<ListInboxPreviewClientsError>> {

    let uri_str = format!("{}/v1/design_studio/inbox_previews/clients", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListInboxPreviewClients200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListInboxPreviewClients200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListInboxPreviewClientsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a paginated history of inbox preview jobs across your workspace, or for a single email when you pass `node_id`. Only lists runs recent enough for their screenshots to still be viewable—an older run won't appear here even though it still exists. 
pub async fn list_inbox_preview_jobs(configuration: &configuration::Configuration, node_id: Option<&str>, limit: Option<i64>, cursor: Option<&str>) -> Result<models::ListInboxPreviewJobs200Response, Error<ListInboxPreviewJobsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_node_id = node_id;
    let p_query_limit = limit;
    let p_query_cursor = cursor;

    let uri_str = format!("{}/v1/design_studio/inbox_previews/jobs", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_node_id {
        req_builder = req_builder.query(&[("node_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_cursor {
        req_builder = req_builder.query(&[("cursor", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListInboxPreviewJobs200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListInboxPreviewJobs200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListInboxPreviewJobsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns an email's saved versions, newest first. A version is a named checkpoint of the email's content and envelope.  - Design Studio creates a version automatically every time you [publish](/integrations/api/app/tag/design-studio-emails/publishEmail/) the email. - You can save a version manually with [Save an email version](/integrations/api/app/tag/design-studio-emails/saveVersion/).  Pass `start_date` to filter for versions created after a certain time. Or pass `start_date` and `end_date` together to filter for versions created in a certain time range.  You can retrieve your email's UUID through [List emails](/integrations/api/app/tag/design-studio-emails/listEmails/). If your email has language variants, *List emails* only returns the UUID for the default language. Use this default ID to retrieve variant IDs through [List email translations](/integrations/api/app/tag/design-studio-emails/listEmailTranslations/). 
pub async fn list_versions(configuration: &configuration::Configuration, id: &str, start_date: Option<i64>, end_date: Option<i64>) -> Result<models::ListVersions200Response, Error<ListVersionsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_query_start_date = start_date;
    let p_query_end_date = end_date;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/versions", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_start_date {
        req_builder = req_builder.query(&[("start_date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_end_date {
        req_builder = req_builder.query(&[("end_date", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListVersions200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListVersions200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListVersionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns an email with liquid fully evaluated against data you provide. The response includes the final HTML as a recipient would see it, along with liquid errors.  Omit the body or send `{}` to render your fallback values: a variable with a `default` filter renders its fallback, and each variable without one reports an error in the response's `errors` object.  The render, preview, review, link, and publish endpoints share a rate limit of 5 requests per second per workspace. 
pub async fn preview_email(configuration: &configuration::Configuration, id: &str, email_preview_request: Option<models::EmailPreviewRequest>) -> Result<models::EmailPreviewResponse, Error<PreviewEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_email_preview_request = email_preview_request;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/preview", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_email_preview_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EmailPreviewResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EmailPreviewResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PreviewEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Publish a linked Design Studio email. This pushes its current content to the workflow it's linked to. You must link an email to a workflow before you can publish it.  Publishing pushes updates to all language variants: the default language and all of the translations. For a single-language email, the publish runs inline and returns `\"status\": \"done\"` with the resulting template version mappings. For a multi-language email, the publish may take longer; if it doesn't finish within the request window, the response returns `\"status\": \"pending\"` along with a `publish_id` you can poll with [Get publish status](/integrations/api/app/tag/design-studio-emails/getPublishStatus/). 
pub async fn publish_email(configuration: &configuration::Configuration, id: &str) -> Result<models::PublishResponse, Error<PublishEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/publish", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PublishResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PublishResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PublishEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns email-ready HTML. Use this endpoint to inspect your compiled markup.  Liquid tags are left intact—`{{customer.first_name}}` stays in the output as written. To evaluate liquid against real profile data instead, use [Preview an email](/integrations/api/app/tag/design-studio-emails/previewEmail/).  The render, preview, review, link, and publish endpoints share a rate limit of 5 requests per second per workspace. 
pub async fn render_email(configuration: &configuration::Configuration, id: &str) -> Result<models::EmailRenderResponse, Error<RenderEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/render", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EmailRenderResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EmailRenderResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RenderEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the email to a saved version. This restores only the email itself; if the code for a referenced custom component changed since this version was saved, the latest component code renders in Design Studio, not the original.  This only edits the draft in Design Studio. If you already linked the email to a workflow, you need to [publish it](/integrations/api/app/tag/design-studio-emails/publishEmail/) to push the restored content live. 
pub async fn restore_version(configuration: &configuration::Configuration, id: &str, version_id: &str) -> Result<models::RestoreVersion200Response, Error<RestoreVersionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_version_id = version_id;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/versions/{version_id}/restore", configuration.base_path, id=crate::app::apis::urlencode(p_path_id), version_id=crate::app::apis::urlencode(p_path_version_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::RestoreVersion200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::RestoreVersion200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RestoreVersionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Checks an email's saved content for errors and suggests how to improve accessibility, styling, and more. This runs the same process as the [*Review* panel in Design Studio](/messaging/design-studio/emails/qa-in-design-studio/) and returns a readiness score, a status per check, and how to fix issues.  This endpoint reviews **a single translation** based on the `:id` you pass. Call this endpoint for each language variant to review each translation.  **Checks**   - `liquid` — Liquid syntax: broken tags, invalid filters, unclosed blocks. Does not report missing variables; use the [preview endpoint](/integrations/api/app/tag/design-studio-emails/previewEmail/) with sample data to validate variable resolution.   - `failed-components` — Custom components that fail to compile. Always `skipped` on the API; it's a browser-runtime-only signal.   - `source` — Raw markup the editor stores, before liquid rendering. Checks for reserved internal HTML attributes, multiple root elements, and design-token clashes.   - `links` — Broken URLs, validated over HTTP (capped at 100 links per review).   - `images` — Broken image URLs and missing alt text (capped at 100 images per review).   - `accessibility` — WCAG accessibility issues.   - `spam` — SpamAssassin score.   - `unsubscribe` — Presence of an unsubscribe link.   - `implied-links` — Bare URLs or email addresses that should be wrapped in `<a>` tags.   - `html-clip` — Gmail's 102 KB clipping threshold.   - `preheader` — Preheader text presence and quality.  The render, preview, review, link, and publish endpoints share a rate limit of 5 requests per second per workspace. 
pub async fn review_email(configuration: &configuration::Configuration, id: &str) -> Result<models::EmailReviewResponse, Error<ReviewEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/review", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::EmailReviewResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::EmailReviewResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ReviewEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Saves a version of the email's current content, envelope, and any component dependencies it references.  If you need to revert to a previous version, use [Restore an email version](/integrations/api/app/tag/design-studio-emails/restoreVersion/).  If the email is identical to the most recent version, this returns that version with `\"created\": false` instead of saving a duplicate. If the code of a referenced custom component changes, you can save a new version of the email with the latest component changes; however, the code diff in Design Studio will show no change because the component code isn't compiled in that view. 
pub async fn save_version(configuration: &configuration::Configuration, id: &str, save_version_request: models::SaveVersionRequest) -> Result<models::SaveVersion200Response, Error<SaveVersionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_save_version_request = save_version_request;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/versions", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_save_version_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SaveVersion200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SaveVersion200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SaveVersionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Initiate one or more inbox previews for an email. This uses inbox preview credits; check your balance with [Get preview credit balance](/integrations/api/app/tag/design-studio-emails/getInboxPreviewCredits/). [**Learn how billing for inbox previews work before you call this endpoint.**](/accounts/billing/inbox-previews/)  Use [List emails](/integrations/api/app/tag/design-studio-emails/listEmails/) to get your email's ID. If the email has translations, call [List email translations](/integrations/api/app/tag/design-studio-emails/listEmailTranslations/) to get the ID of the language variant you want a preview of.  You can retrieve client IDs from [List inbox preview options](/integrations/api/app/tag/design-studio-emails/listInboxPreviewClients/).  This call does not return the preview file. It returns a `run_id`; poll [Get an inbox preview job](/integrations/api/app/tag/design-studio-emails/getInboxPreviewJob/) with it, then fetch each tile's capture URL from that response.  Submitting the same email, content, and devices again on the same UTC day returns the existing run instead of starting a new one, with `replayed` set to `true` and no second charge. The same happens the next day if that run is still processing, or if a concurrent identical request gets there first. Resubmitting a stuck run is safe and free—it hands you the same run back rather than starting a second one. 
pub async fn submit_inbox_preview(configuration: &configuration::Configuration, id: &str, submit_inbox_preview_request: models::SubmitInboxPreviewRequest) -> Result<models::SubmitInboxPreview200Response, Error<SubmitInboxPreviewError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_submit_inbox_preview_request = submit_inbox_preview_request;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/inbox_previews", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_submit_inbox_preview_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SubmitInboxPreview200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SubmitInboxPreview200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SubmitInboxPreviewError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Sends a test email to a real inbox through your sending domain using the content saved to Design Studio. The test includes the tracking pixel and link parameters a real send would add, so you can see your email as it would appear in an actual email client. To get the rendered HTML back in the API response instead, use [Preview an email](/integrations/api/app/tag/design-studio-emails/previewEmail/).  * **If you already linked the email to a workflow**, then keep in mind that what's *saved* to Design Studio may differ from what's *published* to the linked workflow. Make sure you publish your changes when you're ready to push them to your linked workflow.  * **If your email has translations**, you have to send this request separately for each language variant. Pass the ID of the specific translation you want to test, which you can retrieve from [List email translations](/integrations/api/app/tag/design-studio-emails/listEmailTranslations/).  * **If the email contains liquid variables**, you can provide sample data in the request body to check how the liquid renders. Without sample data, the send still succeeds, but each variable renders as an empty string and the response includes a warning. A variable with a fallback filter like `default` renders its fallback instead. A liquid syntax error, like a missing close tag, always fails the request.  Your account has a set number of test emails you can send per day. This endpoint counts towards that quota. Learn more in [Plan features](/accounts/billing/plan-features/#general-features). 
pub async fn test_send_email(configuration: &configuration::Configuration, id: &str, test_send_email_request: models::TestSendEmailRequest) -> Result<models::TestSendEmail200Response, Error<TestSendEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_test_send_email_request = test_send_email_request;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/test_send", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_test_send_email_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::TestSendEmail200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::TestSendEmail200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TestSendEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update part of an email: an email's name, template status, folder, content, envelope, or transformers. Note, this does not publish your email; if the email is linked to a workflow like an automation, you still need to click publish to make the changes live. 
pub async fn update_email(configuration: &configuration::Configuration, id: &str, email_update: models::EmailUpdate) -> Result<(), Error<UpdateEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_body_email_update = email_update;

    let uri_str = format!("{}/v1/design_studio/emails/{id}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_email_update);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update part of an email translation: the content, envelope, or transformers for a specific email translation. Note, this does not publish your email; if the email is linked to a workflow like an automation, you still need to click publish to make the changes live. 
pub async fn update_email_translation(configuration: &configuration::Configuration, id: &str, language: &str, update_email_translation_request: models::UpdateEmailTranslationRequest) -> Result<(), Error<UpdateEmailTranslationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;
    let p_path_language = language;
    let p_body_update_email_translation_request = update_email_translation_request;

    let uri_str = format!("{}/v1/design_studio/emails/{id}/languages/{language}", configuration.base_path, id=crate::app::apis::urlencode(p_path_id), language=crate::app::apis::urlencode(p_path_language));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_email_translation_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateEmailTranslationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

