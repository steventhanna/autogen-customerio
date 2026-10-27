/*
 * Customer.io Track API
 *
 * # Overview  Our Track API provides ways to send real-time customer data to your Customer.io workspace including customer identification and event tracking.  # Use our Postman collection  We've generated a Postman collection to help you get started with our APIs.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  **NOTE**: Postman endpoints default to our US APIs. If you're in our European (EU) region, you'll need to add `-eu` to the server variables (`track_api_url` and `app_api_url`).  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-0f7ae1e8-8177-46fc-808a-2fd363dd52b9?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-0f7ae1e8-8177-46fc-808a-2fd363dd52b9%26entityType%3Dcollection%26workspaceId%3Db886877f-fc09-475f-84fe-6221a98f4d18#?env%5BCustomer.io%20API%20Environment%5D=W3sia2V5IjoidHJhY2tfYXBpX3VybCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiJ0cmFjay5jdXN0b21lci5pbyIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiYXBpLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzaXRlX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBpX2tleSIsInR5cGUiOiJzZWNyZXQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYmVhcmVyIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYnJvYWRjYXN0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaW1wb3J0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiZW1haWxfYWRkcmVzcyIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN1cHByZXNzaW9uX3R5cGUiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjb2xsZWN0aW9uX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoic25pcHBldF9uYW1lIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoid2ViaG9va19pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InNlbmRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImV4cG9ydF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6Im1lc3NhZ2VfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzZWdtZW50X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoibmV3c2xldHRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImNvbnRlbnRfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjYW1wYWlnbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImFjdGlvbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImN1c3RvbWVyX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoidHJhbnNhY3Rpb25hbF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRyaWdnZXJfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJmb3JtX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaWRlbnRpZmllciIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRldmljZV9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRlbGl2ZXJ5X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9XQ==)  # Server addresses: US and EU Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://track.customer.io | | EU | https://track-eu.customer.io |  Note that if your account is in the EU region and you send traffic to our US endpoints, we'll redirect it accordingly but this traffic still passes through US servers and data could be logged in the US.  # Authentication   You can find all of your API authentication information in your [Account Settings](https://fly.customer.io/settings/api_credentials). Our Tracking API uses HTTP basic authorization. The App API uses bearer authorization, and you can generate tokens supporting different scopes. Each operation in this document references the authorization header it requires.  # v1 vs v2 APIs  Most of the time, when we talk about *The Track API*, we're talking about the v1 API because the v2 API isn't used in any of our libraries and rarely used in libraries built by third parties; it's much more common that you'd encounter the v1 API.  If you're integrating with Customer.io using one of our libraries, or a third party customer data platform (CDP) like Segment or Rudderstack, you'll be using the v1 API.  The v2 API is newer and supports two important features that the v1 API doesn't natively support: objects and batching. But, if you're integrating directly with our API, we suggest you use the [Pipelines API](/integrations/api/cdp/). The Pipelines API supports both objects, batching, *and* all of our newest integrations and libraries are based on it.  # Rate Limits  The Track API has no fixed rate limit. Customer.io accepts your requests as fast as you send them and doesn't reject or drop them because of volume. Our general guideline is about 3000 requests per 3 seconds, an average of 1000 requests per second. The 3-second window leaves room for short bursts.  Traffic above this level usually comes from high-volume integrations, which our Premium and Enterprise plans support. If your traffic regularly goes above this, we may contact you to learn more about your integration and talk about optimizing it, or about a plan that better suits your volume.  **This guideline is subject to change.**  Below are the payload size limits for the Track API. If any of these limits are too restrictive for your needs, contact support to let us know your situation as we may be able to accommodate special circumstances.   ## Customer limits  These limits apply to people and their attributes, often referred to as \"customers\" in our APIs.  | Data Type | Limit | Description | | -- | -- | -- | | ID | 150 bytes | Max length of a person's ID value | | Attribute Name | 150 bytes | Max length of each attribute name | | Attribute Value | 1000 bytes | Max length of attribute values | | Unique attributes | 300 | Max number of attributes allowed per person or Identify call |  ## Object and relationship limits  Objects (groups) and relationships between people and objects can have their own attributes. Their limits are similar to people (customers).  | Data Type | Limit | Description | | -- | -- | -- | | Object ID | 150 bytes | Max length of a object's ID | | Attribute Names | 150 bytes | Max length of each attribute name | | Attribute Values | 1000 bytes | Max length of attribute values | | Unique attributes | 300 | Max number of attributes allowed per object or relationship | | Total attribute size | 100 Kilobytes | Max size of all attributes associated with an object or relationship |  ## Track API Event limits  These limits apply to events that you'll send with the `/v1/track` call.  | Data Type | Limit | Description | | -- | -- | -- | | Event Name | 100 bytes | Max length of each event name | | Event Data | 100000 bytes | Max length of each event data |   ## v2 API Limits  The v2 API has two endpoints, both of which have limits on the total size of requests.  * `/entity` is limited to requests 32kb or smaller. * `/batch` is limited to requests 500kb or smaller.      Each of the requests within a batch must also be 32kb or smaller. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::track::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`add_device`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddDeviceError {
    Status400(models::AddDevice400Response),
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_device`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteDeviceError {
    Status400(models::AddDevice400Response),
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`identify`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdentifyError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`merge`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MergeError {
    Status400(models::Merge400Response),
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`suppress`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SuppressError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unsubscribe`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnsubscribeError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unsuppress`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnsuppressError {
    Status401(),
    UnknownValue(serde_json::Value),
}


/// Customers can have more than one device. Use this method to add iOS and Android devices to, or update devices for, a customer profile.
pub async fn add_device(configuration: &configuration::Configuration, identifier: &str, add_device_request: Option<models::AddDeviceRequest>) -> Result<(), Error<AddDeviceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_identifier = identifier;
    let p_body_add_device_request = add_device_request;

    let uri_str = format!("{}/api/v1/customers/{identifier}/devices", configuration.base_path, identifier=p_path_identifier.to_string());
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_body_add_device_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<AddDeviceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Deleting a customer removes them, and all of their information, from Customer.io.  **NOTE**: Calls that update customers by ID can also create a customer. If you send data to Customer.io through other means (like the Javascript snippet), after you delete a customer, you may accidentally recreate the customer. You cannot delete a customer using the Javascript snippet alone. 
pub async fn delete(configuration: &configuration::Configuration, identifier: &str) -> Result<(), Error<DeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_identifier = identifier;

    let uri_str = format!("{}/api/v1/customers/{identifier}", configuration.base_path, identifier=p_path_identifier.to_string());
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Remove a device from a customer profile. If you continue sending data about a device to Customer.io, you may inadvertently re-add the device to the customer profile.
pub async fn delete_device(configuration: &configuration::Configuration, identifier: &str, device_id: &str) -> Result<(), Error<DeleteDeviceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_identifier = identifier;
    let p_path_device_id = device_id;

    let uri_str = format!("{}/api/v1/customers/{identifier}/devices/{device_id}", configuration.base_path, identifier=p_path_identifier.to_string(), device_id=crate::track::apis::urlencode(p_path_device_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteDeviceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Adds or updates a person.  If your request does _not_ include `cio_id` and the identifiers in the request body do not belong to a person, your request adds a person.  If a person already exists with the identifier in the request path, your request updates that person. If the identifier in the path does not belong to a person but you use an identifier in your request body that _does_ belong to a person, your request updates the person and assigns them the identifier in the path.  If the identifier in the path and request body belong to different people, your request may return `200 OK` but produce an *Attribute Update Failure* for the identifier in the payload.  If you want to update a person's identifiers after they are set, you must reference them using their `cio_id` in the format `cio_<cio_id_value>`—unless when updating an `email` with the [Allow updates to email using ID](/accounts/workspaces#update-email-with-id) setting enabled. You can get the `cio_id` value from the [App API](/integrations/api/app/tag/customers/). If your request includes a `cio_id`, we'll attempt to update that person, including any identifiers in the request. If the `cio_id` does not exist or belongs to a person who was deleted, we'll drop the request.  For workspaces using `email` as an identifier, `email` is case-insensitive. The addresses `person@example.com` and `PERSON@example.com` would represent the same person.  **Tip**: If your workspace identifies people by both `email` and `id`, and you send an identify call with a new `id` but an `email` that already belongs to someone, we update the existing person rather than creating a new one. The existing person gets the new `id`. This is a common source of confusion during testing—if you're generating new IDs but reusing the same email address, you're updating one person repeatedly, not creating multiple people. 
pub async fn identify(configuration: &configuration::Configuration, identifier: &str, identify_request: Option<models::IdentifyRequest>) -> Result<(), Error<IdentifyError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_identifier = identifier;
    let p_body_identify_request = identify_request;

    let uri_str = format!("{}/api/v1/customers/{identifier}", configuration.base_path, identifier=p_path_identifier.to_string());
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_body_identify_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<IdentifyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Merge two customer profiles together. The payload contains `primary` and `secondary` profile objects. The primary profile remains after the merge and the secondary is deleted. This operation is _not_ reversible.   The primary profile must already exist in Customer.io for the merge operation to work. If the primary profile doesn't exist, your request won't do anything.   If you perform requests concurrently or in rapid succession, you could create a race condition where the primary profile doesn't exist yet. For example, if you identify a person and send a request to this endpoint immediately, the primary profile might not exist when we process your merge request.  The following information is merged into the primary profile from the secondary profile: * Attributes that are not set, or are empty, on the primary. * The most recent 30-days of event history. Events merged from the secondary person cannot trigger automations. * Manual segments that the primary person did not already belong to. * Message delivery history.  * Automation journeys that the primary person has not entered. If the secondary person has started a journey that the primary person has not, the primary person continues on that automation journey after the merge. If the secondary person has completed journeys that the primary person has not, the primary person gains these historical journeys after the merge. This may be important for determining entry (or re-entry) criteria for subsequent automations, segments, etc. 
pub async fn merge(configuration: &configuration::Configuration, merge_request: Option<models::MergeRequest>) -> Result<(), Error<MergeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body_merge_request = merge_request;

    let uri_str = format!("{}/api/v1/merge_customers", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    req_builder = req_builder.json(&p_body_merge_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<MergeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Delete a customer profile and prevent the person's identifier(s) from being re-added to your workspace. Any future API calls or operations referencing the specified ID are ignored. If you suppress a person in a workspace that identifies people by *email or ID* and both identifiers are set, both the person's email and ID are suppressed.  <div class=\"fly-panel bg-warning\"> <div class=\"fly-panel-body\"> <p class=\"callout-head text--bold text-warning mrg-t-none\"><svg class=\"api-icon\"><path fill-rule=\"evenodd\" clip-rule=\"evenodd\" d=\"M15.4127 13.3333L9.18133 1.43333C8.95116 0.994094 8.49623 0.718884 8.00033 0.718884C7.50444 0.718884 7.04951 0.994094 6.81933 1.43333L0.587332 13.3333C0.370821 13.7467 0.386147 14.2431 0.627743 14.6423C0.869339 15.0415 1.30205 15.2854 1.76867 15.2853H14.2313C14.698 15.2854 15.1307 15.0415 15.3723 14.6423C15.6139 14.2431 15.6292 13.7467 15.4127 13.3333ZM7.33333 5.61533C7.33333 5.24714 7.63181 4.94867 8 4.94867C8.36819 4.94867 8.66667 5.24714 8.66667 5.61533V9.61533C8.66667 9.98352 8.36819 10.282 8 10.282C7.63181 10.282 7.33333 9.98352 7.33333 9.61533V5.61533ZM8.01466 13.2887H8.03333C8.29806 13.2844 8.54988 13.1735 8.73182 12.9812C8.91376 12.7888 9.01044 12.5312 9 12.2667C8.97854 11.7209 8.53019 11.2893 7.984 11.2887H7.96533C7.70125 11.2935 7.4502 11.4043 7.26865 11.5961C7.0871 11.788 6.99029 12.0447 7 12.3087C7.02073 12.8546 7.46838 13.2869 8.01466 13.2887Z\" /></svg>&nbsp;This API permanently deletes people</p> <div class=\"text-warning\"><p>Suppressing a person way deletes their profile <i>and</i> suppresses the identifier you reference in the path of this call, preventing you from re-adding a person using the same identifier (until you unsuppress the identifier). You cannot recover a profile after you suppress it. In general, should use this API sparingly—for GDPR/CCPA requests, etc. </p> <p>If you want to keep a record of a person but prevent them from receiving messages, you should set the person's unsubscribed attribute (or use other attributes to represent complex subscription preferences) instead.</p></div> </div> </div> 
pub async fn suppress(configuration: &configuration::Configuration, identifier: &str) -> Result<(), Error<SuppressError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_identifier = identifier;

    let uri_str = format!("{}/api/v1/customers/{identifier}/suppress", configuration.base_path, identifier=p_path_identifier.to_string());
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<SuppressError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// This endpoint lets you set a global unsubscribed status outside of the subscription pathways native to Customer.io. If you use [custom unsubscribe links](/journeys/people/multiple-subscription-types), you can host a custom unsubscribe page and use this API to send unsubscribe data, associated with a particular delivery, to Customer.io.  **NOTE**: This endpoint **requires** a `Content-type: application/json` header. This endpoint **does not require** an `Authorization` header.  Your request sets a person's `unsubscribed` attribute to `true`, attributes their unsubscribe request to the individual email/delivery that they unsubscribed from, and lets you segment your audience based on `email_unsubscribed` events when you use a custom subscription center.  If you use a custom subscription center (managing subscriptions to various types of messages with custom attributes), this request *does not* set a custom attribute. You must perform a [separate request](/integrations/api/track/tag/track-customers/identify/) to update a person's custom subscription attributes. 
pub async fn unsubscribe(configuration: &configuration::Configuration, delivery_id: &crate::ids::DeliveryId, unsubscribe_request: Option<models::UnsubscribeRequest>) -> Result<(), Error<UnsubscribeError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_delivery_id = delivery_id;
    let p_body_unsubscribe_request = unsubscribe_request;

    let uri_str = format!("{}/unsubscribe/{delivery_id}", configuration.base_path, delivery_id=crate::track::apis::urlencode(p_path_delivery_id));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_body_unsubscribe_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UnsubscribeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Unsuppressing a profile allows you to add the customer back to Customer.io. If you unsuppress a person in a workspace that identifies people by *email or ID* and the suppressed person had both an email and ID, both the person's email and ID are unsuppressed.  Unsuppressing a profile does not recreate the profile that you previously suppressed. Rather, it just makes the identifier available again. Identifying a person after unsuppressing them creates a new profile, with none of the history of the previously suppressed identifier. 
pub async fn unsuppress(configuration: &configuration::Configuration, identifier: &str) -> Result<(), Error<UnsuppressError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_path_identifier = identifier;

    let uri_str = format!("{}/api/v1/customers/{identifier}/unsuppress", configuration.base_path, identifier=p_path_identifier.to_string());
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UnsuppressError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

