/*
 * Customer.io App API
 *
 * Our App API provides ways to trigger messages and retrieve information about people, automations, broadcasts, and more.  # Overview  The App API provides methods to send newsletters, transactional messages, and API-triggered broadcasts. You can create newsletters from scratch and update transactional messages and API-triggered broadcasts.  For transactional messages and API-triggered broadcasts, your payload acts as a message \"trigger\" and can contain `data` that you reference in your messages using liquid—`{{trigger.<data>}}`.  The other endpoints help you retrieve information about people, segments, automations, broadcasts, etc; it also lets you update automation actions, messages, newsletter variants, etc. Aside from the [API-triggered broadcast](/integrations/api/app/tag/send-messages/triggerBroadcast/) endpoint (1 per 10 seconds), requests are limited to 10 per second. [Transactional](/integrations/api/app/tag/send-messages/sendEmail/) messages aren't rate limited.  # We may add to these APIs  As we release new features, we add endpoints and we may even add fields to existing endpoints. We don't announce these additions in advance. Fields we add will be backward compatible: existing fields keep their names and types.  Build your integration to ignore fields it doesn't recognize. If you validate responses against a schema, make sure the schema accepts unknown keys. In JSON Schema, for example, don't set `additionalProperties: false`.  # Use our Postman collection  We've generated a Postman collection to help you get started with our APIs.  If you fork this collection, you might want to disable the *Watch original collection* option. We automatically update our Postman collection whenever we release changes to our documentation, even if we don't change our APIs—which happens daily! Rather than being flooded with Postman notifications, you can check out our [Release Notes](/release-notes/) for updates to our APIs.  **NOTE**: Postman endpoints default to our US APIs. If you're in our European (EU) region, you'll need to add `-eu` to the server variables (`track_api_url` and `app_api_url`).  [<img src=\"https://run.pstmn.io/button.svg\" alt=\"Run In Postman\" style=\"width: 128px; height: 32px;\">](https://god.gw.postman.com/run-collection/23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d?action=collection%2Ffork&source=rip_markdown&collection-url=entityId%3D23697545-2931c004-e63d-4cdc-bf4b-e685ba6da42d%26entityType%3Dcollection%26workspaceId%3Db886877f-fc09-475f-84fe-6221a98f4d18#?env%5BCustomer.io%20API%20Environment%5D=W3sia2V5IjoidHJhY2tfYXBpX3VybCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiJ0cmFjay5jdXN0b21lci5pbyIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBwX2FwaV91cmwiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiYXBpLmN1c3RvbWVyLmlvIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzaXRlX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYXBpX2tleSIsInR5cGUiOiJzZWNyZXQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYmVhcmVyIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiYnJvYWRjYXN0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaW1wb3J0X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiZW1haWxfYWRkcmVzcyIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InN1cHByZXNzaW9uX3R5cGUiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjb2xsZWN0aW9uX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoic25pcHBldF9uYW1lIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5Ijoid2ViaG9va19pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InNlbmRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImV4cG9ydF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6Im1lc3NhZ2VfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJzZWdtZW50X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoibmV3c2xldHRlcl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImNvbnRlbnRfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJjYW1wYWlnbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImFjdGlvbl9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImN1c3RvbWVyX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoidHJhbnNhY3Rpb25hbF9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6InRyaWdnZXJfaWQiLCJ0eXBlIjoiZGVmYXVsdCIsInZhbHVlIjoiIiwiZW5hYmxlZCI6dHJ1ZX0seyJrZXkiOiJmb3JtX2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9LHsia2V5IjoiaWRlbnRpZmllciIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRldmljZV9pZCIsInR5cGUiOiJkZWZhdWx0IiwidmFsdWUiOiIiLCJlbmFibGVkIjp0cnVlfSx7ImtleSI6ImRlbGl2ZXJ5X2lkIiwidHlwZSI6ImRlZmF1bHQiLCJ2YWx1ZSI6IiIsImVuYWJsZWQiOnRydWV9XQ==)  # Server addresses: US and EU  Customer.io hosts services in the United States (US) and European Union. Select the appropriate server address for your region.  | Region | Server Address | | :-- | :-- | | US | https://api.customer.io | | EU | https://api-eu.customer.io |  # Authentication  All requests to the Customer.io App API use a private key (`sk_`) as a bearer token in the HTTP `Authorization` header: `Authorization: Bearer <key>`.  Create a private key on the [API Credentials page](https://fly.customer.io/settings/api_credentials?keyType=app). [Learn more about public and private keys](/accounts/settings/managing-credentials/#public-and-private-keys).  If you already use an App API key, you can keep using it with bearer authentication.  # Rate Limits  Most endpoints on this page are limited to 10 requests per second. The exceptions are: * [Transactional messages](/integrations/api/app/tag/send-messages/sendEmail/) aren't rate limited. * The [API-triggered broadcast endpoint](/integrations/api/app/tag/send-messages/triggerBroadcast/) is limited to 1 request every 10 seconds. If you need to trigger broadcasts more often, contact us to talk through your use case.  **Rate limits are subject to change. We may adjust these thresholds to ensure stable performance for all customers.** 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::app::models;
use serde::{Deserialize, Serialize};

/// DeviceObjectCdpCommonAttributes : Attributes that you can reference to segment your audience—like a person's attributes, but specific to a device. These can be either the attributes defined below or custom key-value attributes.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceObjectCdpCommonAttributes {
    /// The delivery status of the last message sent to the device—sent, bounced, or suppressed. An empty string indicates that that the device hasn't received a push yet.
    #[serde(rename = "_last_status", skip_serializing_if = "Option::is_none")]
    pub _last_status: Option<LastStatus>,
    /// The app that the device belongs to—the **Bundle ID / App identifier** you entered with that app's push credentials. If you have more than one app, we use this attribute to send push notifications to the right app. A device without an app identifier receives push notifications you send to your [default app](/messaging/channels/push/getting-started/#the-default-app).
    #[serde(rename = "app_identifier", skip_serializing_if = "Option::is_none")]
    pub app_identifier: Option<String>,
    /// The version of your app that a customer uses. You might target app versions to let people know when they need to update, or expose them to new features when they do.
    #[serde(rename = "app_version", skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    /// The version of the Customer.io SDK in the app.
    #[serde(rename = "cio_sdk_version", skip_serializing_if = "Option::is_none")]
    pub cio_sdk_version: Option<String>,
    /// The device's [IETF language code](/journeys/channels/localization/getting-started/#supported-languages), such as `en-MX` or `es-ES`.
    #[serde(rename = "device_locale", skip_serializing_if = "Option::is_none")]
    pub device_locale: Option<String>,
    /// The model of the device a person uses.
    #[serde(rename = "device_model", skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    /// The operating system, including the version, on the device.
    #[serde(rename = "device_os", skip_serializing_if = "Option::is_none")]
    pub device_os: Option<String>,
    /// If `true`, the device's bluetooth connection is on.
    #[serde(rename = "network_bluetooth", skip_serializing_if = "Option::is_none")]
    pub network_bluetooth: Option<bool>,
    /// If `true`, the device's cellular connection is on.
    #[serde(rename = "network_cellular", skip_serializing_if = "Option::is_none")]
    pub network_cellular: Option<bool>,
    /// If `true`, the device's WiFi connection is on.
    #[serde(rename = "network_wifi", skip_serializing_if = "Option::is_none")]
    pub network_wifi: Option<bool>,
    /// If `\"true\"`, the device is opted-in and can receive push notifications.
    #[serde(rename = "push_enabled", skip_serializing_if = "Option::is_none")]
    pub push_enabled: Option<PushEnabled>,
    /// The height of the device's screen in pixels.
    #[serde(rename = "screen_height", skip_serializing_if = "Option::is_none")]
    pub screen_height: Option<i64>,
    /// The width of the device's screen in pixels.
    #[serde(rename = "screen_width", skip_serializing_if = "Option::is_none")]
    pub screen_width: Option<i64>,
    /// The time zone of the device.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl DeviceObjectCdpCommonAttributes {
    /// Attributes that you can reference to segment your audience—like a person's attributes, but specific to a device. These can be either the attributes defined below or custom key-value attributes.
    pub fn new() -> DeviceObjectCdpCommonAttributes {
        DeviceObjectCdpCommonAttributes {
            _last_status: None,
            app_identifier: None,
            app_version: None,
            cio_sdk_version: None,
            device_locale: None,
            device_model: None,
            device_os: None,
            network_bluetooth: None,
            network_cellular: None,
            network_wifi: None,
            push_enabled: None,
            screen_height: None,
            screen_width: None,
            timezone: None,
        }
    }
}
/// The delivery status of the last message sent to the device—sent, bounced, or suppressed. An empty string indicates that that the device hasn't received a push yet.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LastStatus {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "bounced")]
    Bounced,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "suppressed")]
    Suppressed,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for LastStatus {
    fn default() -> LastStatus {
        Self::Empty
    }
}
/// If `\"true\"`, the device is opted-in and can receive push notifications.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PushEnabled {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    /// Any value not defined in the spec (the API may add values before the spec does).
    #[serde(untagged)]
    UnknownValue(String),
}

impl Default for PushEnabled {
    fn default() -> PushEnabled {
        Self::True
    }
}

