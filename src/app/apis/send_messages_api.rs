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


/// struct for typed errors of method [`cancel_broadcast_trigger`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelBroadcastTriggerError {
    Status400(),
    Status401(),
    Status404(),
    Status409(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`schedule_newsletter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScheduleNewsletterError {
    Status400(models::NewsletterErrorResponse),
    Status404(),
    Status422(models::NewsletterErrorResponse),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendEmailError {
    Status400(models::SendEmail400Response),
    Status403(models::SendEmail400Response),
    Status413(models::SendEmail400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_in_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendInAppError {
    Status400(models::SendEmail400Response),
    Status403(models::SendEmail400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_inbox_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendInboxMessageError {
    Status400(models::SendEmail400Response),
    Status403(models::SendEmail400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_newsletter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendNewsletterError {
    Status400(models::NewsletterErrorResponse),
    Status404(),
    Status422(models::NewsletterErrorResponse),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_push`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendPushError {
    Status400(models::SendEmail400Response),
    Status403(models::SendEmail400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_sms`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendSmsError {
    Status400(models::SendEmail400Response),
    Status403(models::SendEmail400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_whats_app`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendWhatsAppError {
    Status400(models::SendEmail400Response),
    Status403(models::SendEmail400Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`trigger_broadcast`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerBroadcastError {
    Status401(),
    Status404(),
    Status422(models::TriggerBroadcast422Response),
    UnknownValue(serde_json::Value),
}


/// Cancel a trigger you [scheduled](/integrations/api/app/tag/send-messages/triggerBroadcast/) with `scheduled_at`, before it fires.  The request fails if the broadcast has started calculating its audience, is already sending, or has already sent. A trigger with a `rate_limit` is the exception: you can cancel it while it sends. Canceling stops more people from entering the broadcast, and people who already entered continue through your workflow. 
pub async fn cancel_broadcast_trigger(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId, trigger_id: crate::ids::TriggerId) -> Result<models::CancelBroadcastTrigger200Response, Error<CancelBroadcastTriggerError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;
    let p_path_trigger_id = trigger_id;

    let uri_str = format!("{}/v1/campaigns/{broadcast_id}/triggers/{trigger_id}/cancel", configuration.base_path, broadcast_id=p_path_broadcast_id, trigger_id=p_path_trigger_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CancelBroadcastTrigger200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CancelBroadcastTrigger200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CancelBroadcastTriggerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Schedule a one-time send (referred to as a \"newsletter\" in our APIs) to send at a specific time. It must be in a draft state. If it has already been sent, you'll get a `400` error. If it's already scheduled, this endpoint updates the scheduled time.  The response returns the one-time send, so you can read back the schedule you set: `send_state` and `scheduled_at` are always included, and `timezone` and `tz_match_enabled` are included when they apply.  To retrieve the schedule parameters after scheduling with [Get a one-time send](/integrations/api/app/tag/newsletters/getNewsletters/). After you start a send with timezone-match, `scheduled_at` reports the next timezone batch or the remainder of an A/B test instead of the time you requested.  The recipients are fixed when you create the one-time send. If you're not sure who will receive it, you can use the [List one-time sends](/integrations/api/app/tag/newsletters/listNewsletters/) endpoint to get a list of one-time sends and their recipients. 
pub async fn schedule_newsletter(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId, newsletter_schedule_request: models::NewsletterScheduleRequest) -> Result<models::SendNewsletter200Response, Error<ScheduleNewsletterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;
    let p_body_newsletter_schedule_request = newsletter_schedule_request;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/schedule", configuration.base_path, newsletter_id=p_path_newsletter_id);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_newsletter_schedule_request);

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
        let entity: Option<ScheduleNewsletterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Send a transactional email. While not strictly required, we recommend that you include a `transactional_message_id` in your request. If you don't, Customer.io attributes metrics to `\"transactional_message_id\": 1`, so multiple messages can roll up under the same ID.  If this is the first time you send a message with the API, you can include the `auto_create` parameter along with a `transactional_message_id` string to create a record for you.  You can also include a `body`, `subject`, and `from` values to override the message template. Or, if you create your message entirely through the API, you *must* include these values because your `transactional_message_id` won't have any content. See [Examples and API parameters](/journeys/send/transactional/email/#auto-create-transactional-message-records) for more details. 
pub async fn send_email(configuration: &configuration::Configuration, x_workspace_id: Option<i64>, sendemail: Option<models::Sendemail>) -> Result<models::SendEmail200Response, Error<SendEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_x_workspace_id = x_workspace_id;
    let p_body_sendemail = sendemail;

    let uri_str = format!("{}/v1/send/email", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_x_workspace_id {
        req_builder = req_builder.header("X-Workspace-Id", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_sendemail);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SendEmail200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SendEmail200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SendEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Send a transactional in-app message. In-app messages render in your application through the Customer.io SDK to the devices associated with the person you target by `identifiers`.  You send a message using a `transactional_message_id` for an in-app message template that you've set up in the user interface. The `transactional_message_id` can be either the numerical ID for the template or the *Trigger Name* that you assigned the template.  You can find your `transactional_message_id` from the code sample in the **Overview** tab for your transactional message in the user interface, or you can look up a list of your transactional messages through the [App API](/integrations/api/app/tag/transactional/).  **Note**: Your workspace must have in-app messaging enabled. Requests sent to a workspace without the in-app capability return `403`. 
pub async fn send_in_app(configuration: &configuration::Configuration, x_workspace_id: Option<i64>, sendinapp: Option<models::Sendinapp>) -> Result<models::SendEmail200Response, Error<SendInAppError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_x_workspace_id = x_workspace_id;
    let p_body_sendinapp = sendinapp;

    let uri_str = format!("{}/v1/send/in_app", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_x_workspace_id {
        req_builder = req_builder.header("X-Workspace-Id", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_sendinapp);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SendEmail200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SendEmail200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SendInAppError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Send a transactional inbox message. Inbox messages deliver raw JSON payloads to your application through our JavaScript SDK, allowing you to build custom notification centers, message feeds, and other UI components.  You send a message using a `transactional_message_id` for an inbox message template created in the user interface. The `transactional_message_id` can be either the numerical ID for the template or the *Trigger Name* that you assigned the template.  You can find your `transactional_message_id` from the code sample in the **Overview** tab for your transactional message in the user interface, or you can look up a list of your transactional messages through the [App API](/integrations/api/app/tag/transactional/).  **Note**: Inbox messages are currently available for web platforms only and require the Customer.io In-App Plugin to be installed. 
pub async fn send_inbox_message(configuration: &configuration::Configuration, x_workspace_id: Option<i64>, sendinbox: Option<models::Sendinbox>) -> Result<models::SendEmail200Response, Error<SendInboxMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_x_workspace_id = x_workspace_id;
    let p_body_sendinbox = sendinbox;

    let uri_str = format!("{}/v1/send/inbox_message", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_x_workspace_id {
        req_builder = req_builder.header("X-Workspace-Id", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_sendinbox);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SendEmail200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SendEmail200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SendInboxMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Trigger a one-time send (referred to as a \"newsletter\" in our APIs) immediately. It must be in a draft state. If it has already been sent, you'll get a `400` error. The recipients are fixed when you create the one-time send. If you're not sure who will receive it, you can use the [List one-time sends](/integrations/api/app/tag/newsletters/listNewsletters/) endpoint to get a list of one-time sends and their recipients.  To reschedule a one-time send, use the [Schedule a one-time send](/integrations/api/app/tag/send-messages/scheduleNewsletter/) endpoint instead. 
pub async fn send_newsletter(configuration: &configuration::Configuration, newsletter_id: crate::ids::NewsletterId, send_newsletter_request: Option<models::SendNewsletterRequest>) -> Result<models::SendNewsletter200Response, Error<SendNewsletterError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_newsletter_id = newsletter_id;
    let p_body_send_newsletter_request = send_newsletter_request;

    let uri_str = format!("{}/v1/newsletters/{newsletter_id}/send", configuration.base_path, newsletter_id=p_path_newsletter_id);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_send_newsletter_request);

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
        let entity: Option<SendNewsletterError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Send a transactional push. You send a message using a `transactional_message_id` for a transactional push message template composed in the user interface. You can optionally override any of the template values at send time. The `transactional_message_id` can be either the numerical ID for the template or the *Trigger Name* that you assigned the template.  You can find your `transactional_message_id` from the code sample in the **Overview** tab for your transactional message in the user interface, or you can look up a list of your transactional messages through the [App API](/integrations/api/app/tag/transactional/). 
pub async fn send_push(configuration: &configuration::Configuration, x_workspace_id: Option<i64>, sendpush: Option<models::Sendpush>) -> Result<models::SendEmail200Response, Error<SendPushError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_x_workspace_id = x_workspace_id;
    let p_body_sendpush = sendpush;

    let uri_str = format!("{}/v1/send/push", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_x_workspace_id {
        req_builder = req_builder.header("X-Workspace-Id", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_sendpush);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SendEmail200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SendEmail200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SendPushError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Send a transactional SMS message. To send a message, you'll need to provide a `transactional_message_id`. This is either the numerical ID of your transactional message template or the *Trigger Name* that you assigned the template.  You can find your `transactional_message_id` from the code sample in the **Overview** tab for your transactional message in the user interface, or you can look up a list of your transactional messages through the [App API](/integrations/api/app/tag/transactional/). 
pub async fn send_sms(configuration: &configuration::Configuration, x_workspace_id: Option<i64>, sendsms: Option<models::Sendsms>) -> Result<models::SendEmail200Response, Error<SendSmsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_x_workspace_id = x_workspace_id;
    let p_body_sendsms = sendsms;

    let uri_str = format!("{}/v1/send/sms", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_x_workspace_id {
        req_builder = req_builder.header("X-Workspace-Id", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_sendsms);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SendEmail200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SendEmail200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SendSmsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Send a transactional WhatsApp message. To use this endpoint, you must be integrated with [WhatsApp Business](/messaging/send/transactional/whatsapp/#before-you-begin), not Twilio. Then you need to [create the template in the UI and have it approved by WhatsApp/Meta](/messaging/design-studio/whatsapp-design-studio/) before sending it.  To send a message, you need to provide a `transactional_message_id`. This is either the numerical ID of your transactional message template or the *Trigger Name* that you assigned the template.  You can find your `transactional_message_id` from the code sample in the **Overview** tab for your transactional message in the user interface, or you can look up a list of your transactional messages through the [App API](/integrations/api/app/tag/transactional/listTransactional/). 
pub async fn send_whats_app(configuration: &configuration::Configuration, x_workspace_id: Option<i64>, sendwhatsapp: Option<models::Sendwhatsapp>) -> Result<models::SendEmail200Response, Error<SendWhatsAppError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_header_x_workspace_id = x_workspace_id;
    let p_body_sendwhatsapp = sendwhatsapp;

    let uri_str = format!("{}/v1/send/whatsapp", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_header_x_workspace_id {
        req_builder = req_builder.header("X-Workspace-Id", param_value.to_string());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_sendwhatsapp);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SendEmail200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SendEmail200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SendWhatsAppError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Trigger a broadcast and optionally provide data to populate liquid placeholders in the message. To trigger one-time send broadcasts, use the [one-time send endpoint](/integrations/api/app/tag/send-messages/sendNewsletter/).  Broadcasts are optimized to send messages to a large audience and not for one-to-one interactions. Use our [transactional API](/integrations/api/app/tag/send-messages/sendEmail/) or [event-triggered automations](/journeys/send/campaigns/triggers/#event-trigger) to respond to your audience on an individual, one-to-one basis.  **Define your audience**  **The shape of the request depends on how you define your audience**: default (the recipients set in the UI), custom conditions, a list of emails, a list of customer IDs, a map of users, or a data file.  You can narrow any of these audiences, except the default audience, with the `filter` object. This is useful when a file or a list of people is broader than the audience you want for your broadcast.  **You can only trigger broadcasts to send to people you've already added to your workspace.** A broadcast cannot add or identify new people. If you reference people who don't exist in your broadcast audience, the broadcast will fail by default. You can override this behavior by setting the `email_ignore_missing` and/or `id_ignore_missing` flags to `true`. The broadcast will skip over any people who don't exist in your workspace and send to the remaining recipients.  You can reference properties in the `data` object in your broadcast using liquid—`{{trigger.<property_in_data_obj>}}`.  **Send or schedule your broadcast**  By default, we send your broadcast as soon as you trigger it. Set `scheduled_at` to send it at a future time instead. You can [cancel the trigger](/integrations/api/app/tag/send-messages/cancelBroadcastTrigger/) to stop a scheduled send before it starts sending. You can't change a trigger's `scheduled_at` value later; if you need to reschedule your broadcast, you must cancel the trigger and re-trigger the broadcast with the new timestamp.  If your broadcast produces a `422` error, you can [get more information about the errors](/integrations/api/app/tag/broadcasts/broadcastErrors/) to see what went wrong.  **This endpoint is rate-limited to one request every 10 seconds.** After exceeding this, you'll receive a status of `429`. Learn more about [API-triggered broadcast limits](/integrations/api/app/tag/send-messages/) above. If you need to trigger broadcasts more often, contact us to talk through your use case. 
pub async fn trigger_broadcast(configuration: &configuration::Configuration, broadcast_id: crate::ids::BroadcastId, campaign_request: Option<models::CampaignRequest>) -> Result<models::TriggerBroadcast200Response, Error<TriggerBroadcastError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_broadcast_id = broadcast_id;
    let p_body_campaign_request = campaign_request;

    let uri_str = format!("{}/v1/campaigns/{broadcast_id}/triggers", configuration.base_path, broadcast_id=p_path_broadcast_id);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_campaign_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::TriggerBroadcast200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::TriggerBroadcast200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TriggerBroadcastError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

