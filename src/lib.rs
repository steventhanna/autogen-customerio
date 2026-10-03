//! # autogen-customerio
//!
//! Auto-generated, strongly-typed, async Rust client for the
//! [Customer.io APIs](https://docs.customer.io/api/).
//!
//! Every request/response type and API method is generated directly from Customer.io's
//! published OpenAPI specs (`https://docs.customer.io/files/<api>.json`) with
//! [openapi-generator](https://openapi-generator.tech/), so the surface stays faithful to the
//! APIs and updates automatically when a spec changes. A thin hand-written [`CustomerIoClient`]
//! adds region selection and authentication, and hands you a per-API `Configuration` with the
//! correct base URL already set.
//!
//! ## Why one module per API?
//!
//! Customer.io publishes **three independent APIs**, each with its own host, credential, and
//! spec:
//!
//! | Module | API | Auth |
//! |--------|-----|------|
//! | [`track`] | Journeys Track API — identify people, send events, devices, v2 entity/batch | HTTP Basic, Site ID + API key |
//! | [`app`] | Journeys App API — transactional sends, broadcasts, people, campaigns, metrics | Bearer, App API key |
//! | [`pipelines`] | Data Pipelines (CDP) API — identify, track, page, screen, group, alias, batch | HTTP Basic, source API key |
//!
//! Each is vendored into its own top-level module so their (otherwise colliding) model names
//! stay isolated. Every API is a Cargo feature: enabling only what you use skips compiling the
//! rest entirely, models included.
//!
//! ## Quick start
//!
//! ```no_run
//! use autogen_customerio::{CustomerIoClient, PipelinesApiKey, Region};
//!
//! # #[cfg(feature = "pipelines")]
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = CustomerIoClient::new(Region::Us);
//!
//!     // `client.pipelines(..)` returns a `Configuration` pointed at the Data Pipelines API,
//!     // ready to pass to any function in `autogen_customerio::pipelines::apis`.
//!     let _config = client.pipelines(&PipelinesApiKey::new("your-source-api-key"));
//!     Ok(())
//! }
//! # #[cfg(not(feature = "pipelines"))]
//! # fn main() {}
//! ```
//!
//! ## Regions and authentication
//!
//! A workspace lives in the US or EU data center, and every API has a host per region.
//! [`CustomerIoClient`] takes the [`Region`] once; each accessor takes the credential for its
//! API as a distinct type ([`TrackSiteId`] + [`TrackApiKey`], [`AppApiKey`],
//! [`PipelinesApiKey`]), so a key cannot be passed to the wrong API. Credential types never
//! print their value in `Debug` output.
//!
//! You can still override `base_path` on a returned `Configuration` to point at a proxy or a
//! mock server.
//!
//! ## Middleware
//!
//! Every generated `Configuration.client` is a [`reqwest_middleware::ClientWithMiddleware`]. Use
//! [`CustomerIoClient::builder`] to attach middleware — for example a
//! `reqwest_tracing::TracingMiddleware` installed by the application — to every request the
//! client makes:
//!
//! ```no_run
//! use autogen_customerio::{CustomerIoClient, Region};
//!
//! # fn example(my_middleware: impl reqwest_middleware::Middleware) {
//! let client = CustomerIoClient::builder(Region::Us)
//!     .with(my_middleware)
//!     .build();
//! # }
//! ```
//!
//! This crate emits no spans, logs no URLs, and has no `opentelemetry` dependency; it only routes
//! requests through whatever middleware chain you attach.
//!
//! ## Error handling
//!
//! Calls return `Result<T, apis::Error<E>>`, where `E` is the endpoint-specific error enum.
//! Each API module exposes its own `apis::Error`, which separates transport errors,
//! (de)serialization errors, and structured API error responses (carrying the HTTP status and body).
//!
//! ## Feature flags
//!
//! By default all APIs are enabled. To reduce compile time, select only what you need
//! (and a TLS backend — `native-tls` or `rustls`):
//!
//! ```toml
//! [dependencies]
//! autogen-customerio = { version = "0.1", default-features = false, features = ["track", "native-tls"] }
//! ```

#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "pipelines")]
pub mod pipelines;
#[cfg(feature = "track")]
pub mod track;

pub mod client;
pub mod ids;

pub use client::{
    AppApiKey, CustomerIoClient, CustomerIoClientBuilder, PipelinesApiKey, Region, TrackApiKey,
    TrackSiteId,
};

/// Re-exported because the `Middleware` trait is written in terms of `reqwest` types.
pub use reqwest;
/// Re-exported so callers build middleware against the same `reqwest_middleware` version this
/// crate links.
pub use reqwest_middleware;
