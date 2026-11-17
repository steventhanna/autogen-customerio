/*
 * Customer.io App API
 *
 * Our App API provides ways to trigger messages and retrieve information about people, automations, broadcasts, and more.  # Overview  The App API provides methods to send newsletters, transactional messages, and API-triggered broadcasts. You can create newsletters from scratch and update transactional messages and API-triggered broadcasts.  For transactional messages and API-triggered broadcasts, your payload acts as a message \"trigger\" and can contain `data` that you reference in your messages using liquid—`{{trigger.<data>}}`.  The other endpoints help you retrieve information about people, segments, automations, broadcasts, etc; it also lets you update automation actions, messages, newsletter variants, etc. Aside from the [API-triggered broadcast](/integrations/api/app/tag/send-messages/triggerBroadcast/) endpoint (1 per 10 seconds for each workspace), requests are limited to 10 per second. [Transactional](/integrations/api/app/tag/send-messages/sendEmail/) messages aren't rate limited.  # We may add to these APIs  As we release new features, we add endpoints and we may even add fields to existing endpoints. We don't announce these additions in advance. Fields we add will be backward compatible: existing fields keep their names and types.  Build your integration to ignore fields it doesn't recognize. If you validate responses against a schema, make sure the schema accepts unknown keys. In JSON Schema, for example, don't set `additionalProperties: false`.  # Use our Postman collection  We've generated a Postman collection to help you get started with our APIs.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  **NOTE**: Postman endpoints default to our US APIs. If you're in our European (EU) region, you'll need to add `-eu` to the server variables (`track_api_url` and `app_api_url`).  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d%26entityType%3Dcollection%26workspaceId%3Db886877f-fc09-475f-84fe-6221a98f4d18#?env%5BCustomer.io%20API%20Environment%5D=W3sia2V5IjoidHJhY2tfYXBpX3VybCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiJ0cmFjay5jdXN0b21lci5pbyIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiYXBpLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzaXRlX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBpX2tleSIsInR5cGUiOiJzZWNyZXQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYmVhcmVyIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYnJvYWRjYXN0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaW1wb3J0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiZW1haWxfYWRkcmVzcyIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN1cHByZXNzaW9uX3R5cGUiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjb2xsZWN0aW9uX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoic25pcHBldF9uYW1lIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoid2ViaG9va19pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InNlbmRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImV4cG9ydF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6Im1lc3NhZ2VfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzZWdtZW50X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoibmV3c2xldHRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImNvbnRlbnRfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjYW1wYWlnbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImFjdGlvbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImN1c3RvbWVyX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoidHJhbnNhY3Rpb25hbF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRyaWdnZXJfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJmb3JtX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaWRlbnRpZmllciIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRldmljZV9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRlbGl2ZXJ5X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9XQ==)  # Server addresses: US and EU  Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://api.customer.io | | EU | https://api-eu.customer.io |  # Authentication  All requests to the Customer.io App API use a private key (`sk_`) as a bearer token in the HTTP `Authorization` header: `Authorization: Bearer <key>`.  Create a private key on the [API Credentials page](https://fly.customer.io/settings/api_credentials?keyType=app). [Learn more about public and private keys](/accounts/settings/managing-credentials/#public-and-private-keys).  If you already use an App API key, you can keep using it with bearer authentication.  # Rate Limits  Most endpoints on this page are limited to 10 requests per second. The exceptions are: * [Transactional messages](/integrations/api/app/tag/send-messages/sendEmail/) aren't rate limited. * The [API-triggered broadcast endpoint](/integrations/api/app/tag/send-messages/triggerBroadcast/) is limited to 1 request every 10 seconds across all the broadcasts in a workspace. If you need to trigger broadcasts more often, contact us to talk through your use case.  **Rate limits are subject to change. We may adjust these thresholds to ensure stable performance for all customers.** 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::app::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`get_account_audit_log`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountAuditLogError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_workspace_audit_log`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetWorkspaceAuditLogError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_account_audit_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAccountAuditLogsError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_workspace_audit_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListWorkspaceAuditLogsError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}


/// Returns one account audit log entry. For an update, the entry includes the fields that changed. This endpoint requires an [audit log key](/accounts/audit-logs/#get-audit-logs-through-the-api). 
pub async fn get_account_audit_log(configuration: &configuration::Configuration, id: i64) -> Result<models::AuditLogDetailResponse, Error<GetAccountAuditLogError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/account/audit_logs/{id}", configuration.base_path, id=p_path_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AuditLogDetailResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AuditLogDetailResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAccountAuditLogError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns one audit log entry from the workspace your key belongs to. For an update, the entry includes the fields that changed. 
pub async fn get_workspace_audit_log(configuration: &configuration::Configuration, id: i64) -> Result<models::AuditLogDetailResponse, Error<GetWorkspaceAuditLogError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_id = id;

    let uri_str = format!("{}/v1/audit_logs/{id}", configuration.base_path, id=p_path_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AuditLogDetailResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AuditLogDetailResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetWorkspaceAuditLogError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns account-wide activity: sign-ins and failed sign-ins, team members, roles, API keys, and single sign-on (SSO) settings.  This endpoint requires an [audit log key](/accounts/audit-logs/#get-audit-logs-through-the-api). It works only while the team member who created the key is an account admin. Other App API keys return a `403`.  Entries include the IP address of each change and the email address of each team member named. Results cover the audit log history your plan includes. 
pub async fn list_account_audit_logs(configuration: &configuration::Configuration, limit: Option<i64>, sort_order: Option<&str>, cursor: Option<i64>, start_time: Option<chrono::DateTime<chrono::FixedOffset>>, end_time: Option<chrono::DateTime<chrono::FixedOffset>>, user_id: Option<i64>, service_account_id: Option<i64>, ip_addr: Option<&str>, object_type: Option<&str>, object_id: Option<&str>, action: Option<&str>, attribute_name: Option<&str>) -> Result<models::AuditLogListResponse, Error<ListAccountAuditLogsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_limit = limit;
    let p_query_sort_order = sort_order;
    let p_query_cursor = cursor;
    let p_query_start_time = start_time;
    let p_query_end_time = end_time;
    let p_query_user_id = user_id;
    let p_query_service_account_id = service_account_id;
    let p_query_ip_addr = ip_addr;
    let p_query_object_type = object_type;
    let p_query_object_id = object_id;
    let p_query_action = action;
    let p_query_attribute_name = attribute_name;

    let uri_str = format!("{}/v1/account/audit_logs", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_sort_order {
        req_builder = req_builder.query(&[("sort_order", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_cursor {
        req_builder = req_builder.query(&[("cursor", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_time {
        req_builder = req_builder.query(&[("start_time", &param_value.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))]);
    }
    if let Some(ref param_value) = p_query_end_time {
        req_builder = req_builder.query(&[("end_time", &param_value.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))]);
    }
    if let Some(ref param_value) = p_query_user_id {
        req_builder = req_builder.query(&[("user_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_service_account_id {
        req_builder = req_builder.query(&[("service_account_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_ip_addr {
        req_builder = req_builder.query(&[("ip_addr", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_object_type {
        req_builder = req_builder.query(&[("object_type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_object_id {
        req_builder = req_builder.query(&[("object_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_action {
        req_builder = req_builder.query(&[("action", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_attribute_name {
        req_builder = req_builder.query(&[("attribute_name", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AuditLogListResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AuditLogListResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListAccountAuditLogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the activity in the workspace your key belongs to, like changes to automations, segments, and imports. To collect logs from several workspaces, use a key from each one.  This endpoint works with a private key or an [audit log key](/accounts/audit-logs/#get-audit-logs-through-the-api). Entries include the IP address of each change and the email address of each team member named. Results cover the audit log history your plan includes.  On large workspaces, filter by `object_type` and page with `cursor` rather than raising `limit`. 
pub async fn list_workspace_audit_logs(configuration: &configuration::Configuration, limit: Option<i64>, sort_order: Option<&str>, cursor: Option<i64>, start_time: Option<chrono::DateTime<chrono::FixedOffset>>, end_time: Option<chrono::DateTime<chrono::FixedOffset>>, user_id: Option<i64>, service_account_id: Option<i64>, ip_addr: Option<&str>, object_type: Option<&str>, object_id: Option<&str>, action: Option<&str>, attribute_name: Option<&str>, campaign_id: Option<crate::ids::CampaignId>, campaign_type: Option<&str>) -> Result<models::AuditLogListResponse, Error<ListWorkspaceAuditLogsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_query_limit = limit;
    let p_query_sort_order = sort_order;
    let p_query_cursor = cursor;
    let p_query_start_time = start_time;
    let p_query_end_time = end_time;
    let p_query_user_id = user_id;
    let p_query_service_account_id = service_account_id;
    let p_query_ip_addr = ip_addr;
    let p_query_object_type = object_type;
    let p_query_object_id = object_id;
    let p_query_action = action;
    let p_query_attribute_name = attribute_name;
    let p_query_campaign_id = campaign_id;
    let p_query_campaign_type = campaign_type;

    let uri_str = format!("{}/v1/audit_logs", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_sort_order {
        req_builder = req_builder.query(&[("sort_order", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_cursor {
        req_builder = req_builder.query(&[("cursor", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_start_time {
        req_builder = req_builder.query(&[("start_time", &param_value.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))]);
    }
    if let Some(ref param_value) = p_query_end_time {
        req_builder = req_builder.query(&[("end_time", &param_value.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))]);
    }
    if let Some(ref param_value) = p_query_user_id {
        req_builder = req_builder.query(&[("user_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_service_account_id {
        req_builder = req_builder.query(&[("service_account_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_ip_addr {
        req_builder = req_builder.query(&[("ip_addr", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_object_type {
        req_builder = req_builder.query(&[("object_type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_object_id {
        req_builder = req_builder.query(&[("object_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_action {
        req_builder = req_builder.query(&[("action", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_attribute_name {
        req_builder = req_builder.query(&[("attribute_name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_campaign_id {
        req_builder = req_builder.query(&[("campaign_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_query_campaign_type {
        req_builder = req_builder.query(&[("campaign_type", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AuditLogListResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AuditLogListResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListWorkspaceAuditLogsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

