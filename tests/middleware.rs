#![cfg(feature = "pipelines")]

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use reqwest_middleware::{Middleware, Next};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Clone, Default)]
struct Recorder {
    seen: Arc<Mutex<Vec<(String, String)>>>,
}

#[async_trait]
impl Middleware for Recorder {
    async fn handle(
        &self,
        req: reqwest::Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        self.seen.lock().unwrap().push((
            req.method().to_string(),
            req.url().host_str().unwrap_or_default().to_string(),
        ));
        next.run(req, extensions).await
    }
}

#[tokio::test]
async fn middleware_chain_observes_requests() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/identify"))
        .and(header("Authorization", "Basic dGVzdC1rZXk6"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let recorder = Recorder::default();
    let client = autogen_customerio::CustomerIoClient::builder(autogen_customerio::Region::Us)
        .with(recorder.clone())
        .build();
    let mut config = client.pipelines(&autogen_customerio::PipelinesApiKey::new("test-key"));
    config.base_path = server.uri();

    let result =
        autogen_customerio::pipelines::apis::default_api::identify(&config, None, None).await;

    assert!(result.is_ok(), "request should succeed: {result:?}");
    assert_eq!(
        *recorder.seen.lock().unwrap(),
        vec![("POST".to_string(), "127.0.0.1".to_string())]
    );
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}
