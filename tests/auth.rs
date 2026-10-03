// Guards the hand-written auth wiring in src/client.rs and the spec patch in
// scripts/fix-default-security.py. Each Customer.io API uses a different credential scheme; the
// App spec leaves every Design Studio / Assets operation without a security requirement, so
// without the patch those functions would send no `Authorization` header at all.
#![cfg(all(feature = "track", feature = "app", feature = "pipelines"))]

use autogen_customerio::ids::DeliveryId;
use autogen_customerio::{
    AppApiKey, CustomerIoClient, PipelinesApiKey, Region, TrackApiKey, TrackSiteId,
};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn track_uses_basic_auth_with_site_id_and_api_key() {
    let server = MockServer::start().await;
    // base64("site:key")
    Mock::given(method("PUT"))
        .and(path("/api/v1/customers/42"))
        .and(header("Authorization", "Basic c2l0ZTprZXk="))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let client = CustomerIoClient::new(Region::Us);
    let mut config = client.track(&TrackSiteId::new("site"), &TrackApiKey::new("key"));
    config.base_path = server.uri();

    autogen_customerio::track::apis::track_customers_api::identify(&config, "42", None)
        .await
        .expect("identify should succeed");
}

#[tokio::test]
async fn app_uses_bearer_auth() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/messages/dlv_1"))
        .and(header("Authorization", "Bearer app-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .expect(1)
        .mount(&server)
        .await;

    let client = CustomerIoClient::new(Region::Us);
    let mut config = client.app(&AppApiKey::new("app-key"));
    config.base_path = server.uri();

    autogen_customerio::app::apis::messages_api::get_message(
        &config,
        &DeliveryId::new("dlv_1"),
        None,
    )
    .await
    .expect("get_message should succeed");
}

#[tokio::test]
async fn app_operations_without_declared_security_still_send_bearer() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/design_studio/folders"))
        .and(header("Authorization", "Bearer app-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .expect(1)
        .mount(&server)
        .await;

    let client = CustomerIoClient::new(Region::Us);
    let mut config = client.app(&AppApiKey::new("app-key"));
    config.base_path = server.uri();

    autogen_customerio::app::apis::design_studio_api::list_folders(
        &config, None, None, None, None, None, None, None, None, None, None,
    )
    .await
    .expect("list_folders should succeed");
}

#[tokio::test]
async fn pipelines_uses_basic_auth_with_empty_password() {
    let server = MockServer::start().await;
    // base64("key:")
    Mock::given(method("POST"))
        .and(path("/identify"))
        .and(header("Authorization", "Basic a2V5Og=="))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let client = CustomerIoClient::new(Region::Us);
    let mut config = client.pipelines(&PipelinesApiKey::new("key"));
    config.base_path = server.uri();

    autogen_customerio::pipelines::apis::default_api::identify(&config, None, None)
        .await
        .expect("identify should succeed");
}

#[test]
fn us_region_matches_the_spec_servers() {
    // The generated `Configuration::default()` carries each spec's first (US) server. If
    // Customer.io moves a host, this fails and client.rs needs the new US and EU URLs.
    let client = CustomerIoClient::new(Region::Us);
    let track = client.track(&TrackSiteId::new("s"), &TrackApiKey::new("k"));
    let app = client.app(&AppApiKey::new("k"));
    let pipelines = client.pipelines(&PipelinesApiKey::new("k"));
    assert_eq!(
        track.base_path,
        autogen_customerio::track::apis::configuration::Configuration::default().base_path
    );
    assert_eq!(
        app.base_path,
        autogen_customerio::app::apis::configuration::Configuration::default().base_path
    );
    assert_eq!(
        pipelines.base_path,
        autogen_customerio::pipelines::apis::configuration::Configuration::default().base_path
    );
}

#[test]
fn eu_region_uses_eu_hosts() {
    let client = CustomerIoClient::new(Region::Eu);
    assert_eq!(
        client
            .track(&TrackSiteId::new("s"), &TrackApiKey::new("k"))
            .base_path,
        "https://track-eu.customer.io"
    );
    assert_eq!(
        client.app(&AppApiKey::new("k")).base_path,
        "https://api-eu.customer.io"
    );
    assert_eq!(
        client.pipelines(&PipelinesApiKey::new("k")).base_path,
        "https://cdp-eu.customer.io/v1"
    );
}

#[test]
fn credentials_are_redacted_in_debug() {
    let key = AppApiKey::new("super-secret");
    assert_eq!(format!("{key:?}"), "AppApiKey(<redacted>)");
}
