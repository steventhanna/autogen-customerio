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


/// struct for typed errors of method [`create_newsletter_language_variant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNewsletterLanguageVariantError {
    Status400(),
    Status404(),
    Status422(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_newsletter_test_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNewsletterTestGroupError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_newsletter_test_language_variant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNewsletterTestLanguageVariantError {
    Status400(),
    Status404(),
    Status422(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_newsletter_language_variant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNewsletterLanguageVariantError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_newsletter_test_language_variant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNewsletterTestLanguageVariantError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_newsletter_test_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNewsletterTestGroupsError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_newsletter_variant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNewsletterVariantError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_newsletter_variant_translation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNewsletterVariantTranslationError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_newsletter_variant_translation_test`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNewsletterVariantTranslationTestError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_newsletter_variants`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListNewsletterVariantsError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_newsletter_test_translation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateNewsletterTestTranslationError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_newsletter_variant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateNewsletterVariantError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_newsletter_variant_translation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateNewsletterVariantTranslationError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}


/// Add a language variant to a one-time send (referred to as a \"newsletter\" in our APIs).  If you omit optional fields, the values from the default template are copied over untranslated. Make sure you translate all aspects of your default template.  You can't add language variants to a one-time send that has already been sent. You can't manage emails created with the drag-and-drop editor or Design Studio via this endpoint—use [Create an email translation](/integrations/api/design-studio/tag/emails/createEmailTranslation/) for Design Studio emails.  If it has A/B tests, use [Add a translation to a one-time send test group](/integrations/api/app/tag/newsletter-variants/createNewsletterTestLanguageVariant/) instead. 
pub async fn create_newsletter_language_variant(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId, create_language_variant_request: models::CreateLanguageVariantRequest) -> Result<models::SendNewsletter200Response, Error<CreateNewsletterLanguageVariantError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;
    let p_body_create_language_variant_request = create_language_variant_request;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/language", configuration.base_path, newsletter_id=p_path_newsletter_id);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_language_variant_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SendNewsletter200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SendNewsletter200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateNewsletterLanguageVariantError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Create a new A/B test group for a one-time send (referred to as a \"newsletter\" in our APIs). This duplicates the existing one-time send's content into a new test group, allowing you to test different versions of your message.  This endpoint does not require a request body. The new test group is created as a copy of the existing content.  You cannot add test groups to a one-time send that has already been sent. 
pub async fn create_newsletter_test_group(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId, body: Option<serde_json::Value>) -> Result<models::SendNewsletter200Response, Error<CreateNewsletterTestGroupError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;
    let p_body_body = body;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/test_groups", configuration.base_path, newsletter_id=p_path_newsletter_id);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SendNewsletter200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SendNewsletter200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateNewsletterTestGroupError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Add a language variant to a specific A/B test group in a one-time send (referred to as a \"newsletter\" in our APIs). The new variant is a copy of the default template in the test group with the content you provide. The payload the endpoint accepts depends on the parent one-time send's channel type: if it's an email, the payload will be an email variant; if it's an SMS, the payload will be an SMS variant.  You cannot add language variants to a one-time send that has already been sent, or to one-time sends created with the drag-and-drop editor or Design Studio. 
pub async fn create_newsletter_test_language_variant(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId, test_group_id: &str, create_language_variant_request: models::CreateLanguageVariantRequest) -> Result<models::SendNewsletter200Response, Error<CreateNewsletterTestLanguageVariantError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;
    let p_path_test_group_id = test_group_id;
    let p_body_create_language_variant_request = create_language_variant_request;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/test_group/{test_group_id}/language", configuration.base_path, newsletter_id=p_path_newsletter_id, test_group_id=crate::app::apis::urlencode(p_path_test_group_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_language_variant_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SendNewsletter200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SendNewsletter200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateNewsletterTestLanguageVariantError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Delete a specific language variant of a one-time send (referred to as a \"newsletter\" in our APIs). You cannot delete the default language variant. If it has already been sent, you cannot delete language variants.  If it includes A/B tests, use [Delete a translation in a one-time send test group](/integrations/api/app/tag/newsletter-variants/deleteNewsletterTestLanguageVariant/). 
pub async fn delete_newsletter_language_variant(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId, language: &str) -> Result<(), Error<DeleteNewsletterLanguageVariantError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;
    let p_path_language = language;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/language/{language}", configuration.base_path, newsletter_id=p_path_newsletter_id, language=crate::app::apis::urlencode(p_path_language));
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
        let entity: Option<DeleteNewsletterLanguageVariantError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Delete a specific language variant of a one-time send (referred to as a \"newsletter\" in our APIs) in an A/B test group. You cannot delete the default language variant. If it has already been sent, you cannot delete language variants.  You can retrieve a list of `test_group_ids` from [List A/B test groups in a one-time send](/integrations/api/app/tag/newsletter-variants/getNewsletterTestGroups/). 
pub async fn delete_newsletter_test_language_variant(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId, test_group_id: &str, language: &str) -> Result<(), Error<DeleteNewsletterTestLanguageVariantError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;
    let p_path_test_group_id = test_group_id;
    let p_path_language = language;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/test_group/{test_group_id}/language/{language}", configuration.base_path, newsletter_id=p_path_newsletter_id, test_group_id=crate::app::apis::urlencode(p_path_test_group_id), language=crate::app::apis::urlencode(p_path_language));
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
        let entity: Option<DeleteNewsletterTestLanguageVariantError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns information about each test group in a one-time send (referred to as a \"newsletter\" in our APIs), including content ids for each group.
pub async fn get_newsletter_test_groups(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId) -> Result<models::GetNewsletterTestGroups200Response, Error<GetNewsletterTestGroupsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/test_groups", configuration.base_path, newsletter_id=p_path_newsletter_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetNewsletterTestGroups200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetNewsletterTestGroups200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetNewsletterTestGroupsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns information about a specific variant of a one-time send (referred to as a \"newsletter\" in our APIs), where a variant is either a language in a multi-language one-time send or a part of an A/B test.
pub async fn get_newsletter_variant(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId, content_id: crate::ids::ContentId) -> Result<models::GetNewsletterVariant200Response, Error<GetNewsletterVariantError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;
    let p_path_content_id = content_id;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/contents/{content_id}", configuration.base_path, newsletter_id=p_path_newsletter_id, content_id=p_path_content_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetNewsletterVariant200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetNewsletterVariant200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetNewsletterVariantError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns information about a specific language variant of a one-time send (referred to as a \"newsletter\" in our APIs). If it includes A/B tests, use [Get a translation in a one-time send test group](/integrations/api/app/tag/newsletter-variants/getNewsletterVariantTranslationTest/).
pub async fn get_newsletter_variant_translation(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId, language: &str) -> Result<models::GetNewsletterVariant200Response, Error<GetNewsletterVariantTranslationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;
    let p_path_language = language;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/language/{language}", configuration.base_path, newsletter_id=p_path_newsletter_id, language=crate::app::apis::urlencode(p_path_language));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetNewsletterVariant200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetNewsletterVariant200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetNewsletterVariantTranslationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns information about a specific language variant of a one-time send (referred to as a \"newsletter\" in our APIs) in an A/B test group. You can retrieve `test_group_ids` from [List a one-time send's A/B test groups](/integrations/api/app/tag/newsletter-variants/getNewsletterTestGroups/).
pub async fn get_newsletter_variant_translation_test(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId, test_group_id: &str, language: &str) -> Result<models::GetNewsletterVariant200Response, Error<GetNewsletterVariantTranslationTestError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;
    let p_path_test_group_id = test_group_id;
    let p_path_language = language;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/test_group/{test_group_id}/language/{language}", configuration.base_path, newsletter_id=p_path_newsletter_id, test_group_id=crate::app::apis::urlencode(p_path_test_group_id), language=crate::app::apis::urlencode(p_path_language));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetNewsletterVariant200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetNewsletterVariant200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetNewsletterVariantTranslationTestError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a one-time send's content variants (a one-time send is referred to as a \"newsletter\" in our APIs)—these are either different languages in a multi-language one-time send or A/B tests.
pub async fn list_newsletter_variants(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId) -> Result<models::ListNewsletterVariants200Response, Error<ListNewsletterVariantsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/contents", configuration.base_path, newsletter_id=p_path_newsletter_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListNewsletterVariants200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListNewsletterVariants200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListNewsletterVariantsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update the translation of a one-time send (referred to as a \"newsletter\" in our APIs) variant in an A/B test. You can retrieve a list of `test_group_ids` from [List a one-time send's A/B test groups](/integrations/api/app/tag/newsletter-variants/getNewsletterTestGroups/).  **NOTE**: You cannot manage content made with Design Studio with this endpoint. Use the [Design Studio APIs](/integrations/api/design-studio/) instead. You also can't update in-app or inbox message content through the API—change it in the message editor. 
pub async fn update_newsletter_test_translation(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId, test_group_id: &str, language: &str, update_content_request: Option<models::UpdateContentRequest>) -> Result<models::GetNewsletterVariant200Response, Error<UpdateNewsletterTestTranslationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;
    let p_path_test_group_id = test_group_id;
    let p_path_language = language;
    let p_body_update_content_request = update_content_request;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/test_group/{test_group_id}/language/{language}", configuration.base_path, newsletter_id=p_path_newsletter_id, test_group_id=crate::app::apis::urlencode(p_path_test_group_id), language=crate::app::apis::urlencode(p_path_language));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_content_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetNewsletterVariant200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetNewsletterVariant200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateNewsletterTestTranslationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update the content of a one-time send (referred to as a \"newsletter\" in our APIs): the default message, a test variant in an A/B test group, or a translation.  For an email, you can also update the envelope: from address, subject line, etc.  **NOTE**: You cannot manage content made with the drag-and-drop editor via API, and you cannot use this endpoint to update Design Studio emails. You can, however, [manage Design Studio content with other endpoints](/integrations/api/design-studio/). You also can't update in-app or inbox message content through the API—use the message editor instead. 
pub async fn update_newsletter_variant(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId, content_id: crate::ids::ContentId, update_content_request: Option<models::UpdateContentRequest>) -> Result<models::GetNewsletterVariant200Response, Error<UpdateNewsletterVariantError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;
    let p_path_content_id = content_id;
    let p_body_update_content_request = update_content_request;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/contents/{content_id}", configuration.base_path, newsletter_id=p_path_newsletter_id, content_id=p_path_content_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_content_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetNewsletterVariant200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetNewsletterVariant200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateNewsletterVariantError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update the translation of a one-time send (referred to as a \"newsletter\" in our APIs) variant. If it includes A/B tests, use [Update a translation in a one-time send test group](/integrations/api/app/tag/newsletter-variants/updateNewsletterTestTranslation/).  **NOTE**: You cannot manage content made with Design Studio with this endpoint. Use the [Design Studio APIs](/integrations/api/design-studio/) instead. You also can't update in-app or inbox message content through the API—change it in the message editor. 
pub async fn update_newsletter_variant_translation(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId, language: &str, update_content_request: Option<models::UpdateContentRequest>) -> Result<models::GetNewsletterVariant200Response, Error<UpdateNewsletterVariantTranslationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;
    let p_path_language = language;
    let p_body_update_content_request = update_content_request;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/language/{language}", configuration.base_path, newsletter_id=p_path_newsletter_id, language=crate::app::apis::urlencode(p_path_language));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_content_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetNewsletterVariant200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetNewsletterVariant200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateNewsletterVariantTranslationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

