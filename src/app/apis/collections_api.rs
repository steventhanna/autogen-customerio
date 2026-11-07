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


/// struct for typed errors of method [`add_collection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddCollectionError {
    Status400(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`check_collection_source`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckCollectionSourceError {
    Status400(),
    Status404(),
    Status422(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_collection_source`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCollectionSourceError {
    Status400(),
    Status422(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_collection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCollectionError {
    Status404(),
    Status422(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_collection_source`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCollectionSourceError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_collection_sync`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCollectionSyncError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_collection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCollectionError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_collection_contents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCollectionContentsError {
    Status400(),
    Status404(),
    Status409(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_collection_source`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCollectionSourceError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_collection_sync`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCollectionSyncError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_collection_sync_rejects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCollectionSyncRejectsError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_collection_write`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCollectionWriteError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_collections`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCollectionsError {
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_collection_sources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCollectionSourcesError {
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`poll_collection_sync`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PollCollectionSyncError {
    Status400(),
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_collection_sync`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutCollectionSyncError {
    Status400(),
    Status404(),
    Status422(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`submit_collection_write`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitCollectionWriteError {
    Status404(),
    Status422(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_collection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCollectionError {
    Status400(),
    Status409(),
    Status422(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_collection_contents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCollectionContentsError {
    Status404(),
    Status429(models::AppApiError),
    UnknownValue(serde_json::Value),
}


/// Create a new collection and provide the `data` that you'll access from the collection or the `url` that you'll download CSV or JSON data from.  **Note**: A collection that you upload or send in an API request can be up to 10 MB. A collection that you load from files, with write requests or a sync, can be larger: each file can be up to 500 MB and 1,000,000 rows. Each top-level field in a row can hold up to 10,000 characters. The total size of all collections in your account can't exceed 5 GB. Customer.io rejects a request that would take your account over that limit. 
pub async fn add_collection(configuration: &configuration::Configuration, add_collection_request: Option<models::AddCollectionRequest>) -> Result<models::AddCollection200Response, Error<AddCollectionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_add_collection_request = add_collection_request;

    let uri_str = format!("{}/v1/collections", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_add_collection_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AddCollection200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AddCollection200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddCollectionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Check that a source's credentials can still read its bucket, like after you rotate a key or change the bucket's permissions. Customer.io runs the same check as when you connect the source.
pub async fn check_collection_source(configuration: &configuration::Configuration, source_id: &str) -> Result<models::CheckCollectionSource200Response, Error<CheckCollectionSourceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_source_id = source_id;

    let uri_str = format!("{}/v1/collections/sources/{source_id}/check", configuration.base_path, source_id=crate::app::apis::urlencode(p_path_source_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CheckCollectionSource200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CheckCollectionSource200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CheckCollectionSourceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Connect a bucket in Amazon S3, Google Cloud Storage, or Azure Blob Storage so that you can load files from it into your collections. After you connect a source, send its `id` and a file `path` to [write to a collection](/integrations/api/app/tag/collections/submitCollectionWrite/), or [sync a collection](/integrations/api/app/tag/collections/putCollectionSync/) to load new files from a folder automatically.  Your credentials need permission to list and read the bucket:  - **S3:** `s3:ListBucket` and `s3:GetObject`. - **Google Cloud Storage:** the Storage Object Viewer role. - **Azure:** a container SAS URL that grants read and list (`sp=rl`).  Before Customer.io saves the source, it lists the bucket under your `prefix` and reads the first file it finds. If the bucket has no files under the prefix yet, Customer.io can only check that your credentials list the bucket. Customer.io encrypts your credentials and never returns them. 
pub async fn create_collection_source(configuration: &configuration::Configuration, create_collection_source_request: Option<models::CreateCollectionSourceRequest>) -> Result<models::CreateCollectionSource200Response, Error<CreateCollectionSourceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_create_collection_source_request = create_collection_source_request;

    let uri_str = format!("{}/v1/collections/sources", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_create_collection_source_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateCollectionSource200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateCollectionSource200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateCollectionSourceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Remove a collection and its contents. You can't delete a collection that an automation or broadcast queries, including an archived one, because you could unarchive it later. Remove the collection from those messages first.
pub async fn delete_collection(configuration: &configuration::Configuration, collection_id: crate::ids::CollectionId) -> Result<(), Error<DeleteCollectionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_collection_id = collection_id;

    let uri_str = format!("{}/v1/collections/{collection_id}", configuration.base_path, collection_id=p_path_collection_id);
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
        let entity: Option<DeleteCollectionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Disconnect a bucket from your workspace. Collections that you already wrote from the source keep their rows. After you delete a source, Customer.io responds to a new write request that names it with a `422`. A write request that you queued before you deleted the source fails with the code `source-refused`. A sync that reads from the source stops with the `state` `failed`, so delete the sync or point it at another source.
pub async fn delete_collection_source(configuration: &configuration::Configuration, source_id: &str) -> Result<(), Error<DeleteCollectionSourceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_source_id = source_id;

    let uri_str = format!("{}/v1/collections/sources/{source_id}", configuration.base_path, source_id=crate::app::apis::urlencode(p_path_source_id));
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
        let entity: Option<DeleteCollectionSourceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Stop loading files into the collection, and delete the sync's rejected rows. Rows that the sync already loaded stay in the collection.
pub async fn delete_collection_sync(configuration: &configuration::Configuration, collection_id: crate::ids::CollectionId) -> Result<(), Error<DeleteCollectionSyncError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_collection_id = collection_id;

    let uri_str = format!("{}/v1/collections/{collection_id}/sync", configuration.base_path, collection_id=p_path_collection_id);
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
        let entity: Option<DeleteCollectionSyncError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieves details about a collection, including the `schema` and `name`. This request does not include the `content` of the collection (the values associated with keys in the schema).
pub async fn get_collection(configuration: &configuration::Configuration, collection_id: crate::ids::CollectionId) -> Result<models::AddCollection200Response, Error<GetCollectionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_collection_id = collection_id;

    let uri_str = format!("{}/v1/collections/{collection_id}", configuration.base_path, collection_id=p_path_collection_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AddCollection200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AddCollection200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCollectionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve the rows of a collection.  Without query parameters, this request returns the whole collection as JSON Lines: each line of the response is one row, as a JSON object. If a collection is too large to return whole, Customer.io responds with a `400`, and you need to page through it instead. Use `keys` to read specific rows from a collection with a key field, or `limit` and `cursor` to page through the collection. Both return a JSON object with a `rows` array instead. 
pub async fn get_collection_contents(configuration: &configuration::Configuration, collection_id: crate::ids::CollectionId, keys: Option<Vec<String>>, limit: Option<i64>, cursor: Option<&str>) -> Result<models::GetCollectionContents200Response, Error<GetCollectionContentsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_collection_id = collection_id;
    let p_query_keys = keys;
    let p_query_limit = limit;
    let p_query_cursor = cursor;

    let uri_str = format!("{}/v1/collections/{collection_id}/content", configuration.base_path, collection_id=p_path_collection_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_query_keys {
        req_builder = match "csv" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("keys".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("keys", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCollectionContents200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCollectionContents200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCollectionContentsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a collection source, without its credentials.
pub async fn get_collection_source(configuration: &configuration::Configuration, source_id: &str) -> Result<models::CreateCollectionSource200Response, Error<GetCollectionSourceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_source_id = source_id;

    let uri_str = format!("{}/v1/collections/sources/{source_id}", configuration.base_path, source_id=crate::app::apis::urlencode(p_path_source_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateCollectionSource200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateCollectionSource200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCollectionSourceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the collection's sync, including the last file Customer.io loaded and the last error.
pub async fn get_collection_sync(configuration: &configuration::Configuration, collection_id: crate::ids::CollectionId) -> Result<models::GetCollectionSync200Response, Error<GetCollectionSyncError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_collection_id = collection_id;

    let uri_str = format!("{}/v1/collections/{collection_id}/sync", configuration.base_path, collection_id=p_path_collection_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCollectionSync200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCollectionSync200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCollectionSyncError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the rows from synced files that Customer.io couldn't load, sorted by file and then by row. Customer.io keeps rejected rows for 90 days.
pub async fn get_collection_sync_rejects(configuration: &configuration::Configuration, collection_id: crate::ids::CollectionId, limit: Option<i64>, cursor: Option<&str>) -> Result<models::GetCollectionSyncRejects200Response, Error<GetCollectionSyncRejectsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_collection_id = collection_id;
    let p_query_limit = limit;
    let p_query_cursor = cursor;

    let uri_str = format!("{}/v1/collections/{collection_id}/sync/rejects", configuration.base_path, collection_id=p_path_collection_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCollectionSyncRejects200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCollectionSyncRejects200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCollectionSyncRejectsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the status of a change you submitted to a collection and, if the change failed, the reason. Customer.io keeps each result for 24 hours.
pub async fn get_collection_write(configuration: &configuration::Configuration, collection_id: crate::ids::CollectionId, write_id: &str) -> Result<models::SubmitCollectionWrite202Response, Error<GetCollectionWriteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_collection_id = collection_id;
    let p_path_write_id = write_id;

    let uri_str = format!("{}/v1/collections/{collection_id}/writes/{write_id}", configuration.base_path, collection_id=p_path_collection_id, write_id=crate::app::apis::urlencode(p_path_write_id));
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SubmitCollectionWrite202Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SubmitCollectionWrite202Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCollectionWriteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns a list of all of your collections, including the `name` and `schema` for each collection.
pub async fn get_collections(configuration: &configuration::Configuration, ) -> Result<models::GetCollections200Response, Error<GetCollectionsError>> {

    let uri_str = format!("{}/v1/collections", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCollections200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCollections200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCollectionsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the cloud storage buckets that you connected to your workspace as collection sources. Customer.io never returns a source's credentials.
pub async fn list_collection_sources(configuration: &configuration::Configuration, ) -> Result<models::ListCollectionSources200Response, Error<ListCollectionSourcesError>> {

    let uri_str = format!("{}/v1/collections/sources", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ListCollectionSources200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ListCollectionSources200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListCollectionSourcesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Check the sync's folder for new files now, instead of waiting for the next scheduled check. If a check is already running, Customer.io doesn't start another. Customer.io responds right away, before the check finishes. Use [look up a collection's sync](/integrations/api/app/tag/collections/getCollectionSync/) to see the result.
pub async fn poll_collection_sync(configuration: &configuration::Configuration, collection_id: crate::ids::CollectionId) -> Result<(), Error<PollCollectionSyncError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_collection_id = collection_id;

    let uri_str = format!("{}/v1/collections/{collection_id}/sync/poll", configuration.base_path, collection_id=p_path_collection_id);
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

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<PollCollectionSyncError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Watch a folder in one of your [collection sources](/integrations/api/app/tag/collections/createCollectionSource/), and have Customer.io load each new file in the folder into the collection. Customer.io saves and deletes rows by key, the same way as a [write operation](/integrations/api/app/tag/collections/submitCollectionWrite/), so the collection needs a key field. A collection can have exactly one sync.  Each time Customer.io checks the folder, it loads new files in alphabetical order of their paths, up to 2,000 files at a time. Customer.io remembers the last file it loaded and only loads files whose paths sort after it. Name your files so that newer files sort later, like by starting each name with the date (`2026-11-03-products.csv`). Customer.io never loads a file whose path sorts before the last file it loaded, or a file that you overwrite at a path it already loaded. To send changed rows, write a new file.  Customer.io checks each file before it saves any of the file's rows. Customer.io loads the valid rows and records rejected rows, which you can look up with [list rejected sync rows](/integrations/api/app/tag/collections/getCollectionSyncRejects/). If two rows in a file have the same key, Customer.io loads the first one and rejects the second. If Customer.io can't load a whole file, like when the file is too large or isn't JSON or CSV, the sync's `state` becomes `failed`, and Customer.io tries that file again at each later check. To get the sync going again, overwrite the file with one that Customer.io can load, or delete the file.  To change a sync, send it again. If you change only `poll_hours`, the sync keeps its place. If you change the source or the `prefix`, the sync starts over from the first file in the folder. 
pub async fn put_collection_sync(configuration: &configuration::Configuration, collection_id: crate::ids::CollectionId, put_collection_sync_request: Option<models::PutCollectionSyncRequest>) -> Result<models::GetCollectionSync200Response, Error<PutCollectionSyncError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_collection_id = collection_id;
    let p_body_put_collection_sync_request = put_collection_sync_request;

    let uri_str = format!("{}/v1/collections/{collection_id}/sync", configuration.base_path, collection_id=p_path_collection_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_put_collection_sync_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetCollectionSync200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetCollectionSync200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PutCollectionSyncError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Add, update, or delete rows by key, without replacing the whole collection. The collection needs a [key field](/messaging/objects-data/collections/#key-fields), which you can set when you [update a collection](/integrations/api/app/tag/collections/updateCollection/). A row whose key already exists replaces the old row, and a row with a new key is added. To replace every row instead, use [update a collection](/integrations/api/app/tag/collections/updateCollection/).  To delete rows, give the collection a `delete_field`, like `deleted`. A row that sets that field to `true` deletes the row with the same key. A row that doesn't set it to `true` is saved with the field as you sent it. One write request can save some rows and delete others.  Send the rows in one of three ways: in the request as `data`, as a file at a `url`, or as a file in one of your [collection sources](/integrations/api/app/tag/collections/listCollectionSources/) with `source_id` and `path`. Customer.io reads a file when the write runs, so a file can hold far more rows than a request body can.  Customer.io queues the write and responds right away with a write `id`, so large writes don't time out. Customer.io applies writes to a collection one at a time, in the order it receives them. Use [look up a collection write](/integrations/api/app/tag/collections/getCollectionWrite/) to check the result.  Customer.io checks every row before it saves anything. If any row is invalid, Customer.io doesn't save any of them. Customer.io can handle a write of up to 500 rows all at once. If your request is larger, we apply writes 500 rows at a time. While a large write runs, a read can return some of its rows and not others. If a large write fails partway, the rows it saved stay in the collection, and you can safely send the same write again. 
pub async fn submit_collection_write(configuration: &configuration::Configuration, collection_id: crate::ids::CollectionId, submit_collection_write_request: Option<models::SubmitCollectionWriteRequest>) -> Result<models::SubmitCollectionWrite202Response, Error<SubmitCollectionWriteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_collection_id = collection_id;
    let p_body_submit_collection_write_request = submit_collection_write_request;

    let uri_str = format!("{}/v1/collections/{collection_id}/writes", configuration.base_path, collection_id=p_path_collection_id);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_submit_collection_write_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::SubmitCollectionWrite202Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::SubmitCollectionWrite202Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SubmitCollectionWriteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Update the `name` or replace the contents of a collection. Updating the `data` or `url` for your collection fully replaces the contents of the collection.  To set or change the collection's key field or delete field without replacing its rows, send `key_field` or `delete_field` without `data` or `url`. You can send both in one request. To remove either field, send an empty string. Changing either field can mean copying every row, so Customer.io queues the change as a write request and responds with a `202` and the write's `id`. Use [get a write's status](/integrations/api/app/tag/collections/getCollectionWrite/) to see when the change is done. If a row is missing a key or repeats another row's key, the write fails with the code `invalid-data`, and its `message` names the row. If you send a field that's already set, Customer.io doesn't queue anything and responds with a `200`.  When you replace the rows of a large collection, Customer.io changes only the rows that differ, and saves them in batches. While the replacement operation runs, a read can return some new rows alongside old ones. If a large replacement stops partway and can't finish, the collection is left part old and part new. Until you replace the rows again, writes fail with the code `replace-unfinished`.  **Note**: * If you reference your collection by name in active automation messages, changing the name of the collection will cause references to the previous name to return an empty data set. * A collection that you upload or send in an API request can be up to 10 MB. A collection that you load from files, with write requests or a sync, can be larger: each file can be up to 500 MB and 1,000,000 rows. Each top-level field in a row can hold up to 10,000 characters. The total size of all collections in your account can't exceed 5 GB. Customer.io rejects a request that would take your account over that limit. 
pub async fn update_collection(configuration: &configuration::Configuration, collection_id: crate::ids::CollectionId, update_collection_request: Option<models::UpdateCollectionRequest>) -> Result<models::AddCollection200Response, Error<UpdateCollectionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_collection_id = collection_id;
    let p_body_update_collection_request = update_collection_request;

    let uri_str = format!("{}/v1/collections/{collection_id}", configuration.base_path, collection_id=p_path_collection_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_update_collection_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AddCollection200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AddCollection200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateCollectionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Replace the contents of a collection (the `data` from when you created or updated a collection). Send the rows as a JSON array of objects, or as CSV with a `Content-Type` of `text/csv`. This request replaces the current contents of the collection entirely, and Customer.io responds once the change is done.  If you don't want to update the contents directly—you want to change the `name` or data `url` for your collection, use the [update a collection](/integrations/api/app/tag/collections/updateCollection/) endpoint.  **Note**: A collection that you upload or send in an API request can be up to 10 MB. A collection that you load from files, with write requests or a sync, can be larger: each file can be up to 500 MB and 1,000,000 rows. Each top-level field in a row can hold up to 10,000 characters. The total size of all collections in your account can't exceed 5 GB. Customer.io rejects a request that would take your account over that limit. 
pub async fn update_collection_contents(configuration: &configuration::Configuration, collection_id: crate::ids::CollectionId, request_body: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>) -> Result<models::AddCollection200Response, Error<UpdateCollectionContentsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_collection_id = collection_id;
    let p_body_request_body = request_body;

    let uri_str = format!("{}/v1/collections/{collection_id}/content", configuration.base_path, collection_id=p_path_collection_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body_request_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AddCollection200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AddCollection200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateCollectionContentsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

