//! Hand-written entry point that adds region selection and authentication on top of the
//! generated per-service configurations. This is the only file (besides `lib.rs`) that is not
//! generated.

use std::sync::Arc;

/// The Customer.io data center your workspace lives in. Every API has a separate host per
/// region, and credentials only work against their own region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Region {
    /// United States (`track.customer.io`, `api.customer.io`, `cdp.customer.io`).
    Us,
    /// European Union (`track-eu.customer.io`, `api-eu.customer.io`, `cdp-eu.customer.io`).
    Eu,
}

/// Declares a credential newtype: the value is private, construction goes through `new`,
/// and `Debug` never prints the secret.
macro_rules! secret {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Clone, PartialEq, Eq)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Wrap a raw value as a [`", stringify!($name), "`].")]
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(concat!(stringify!($name), "(<redacted>)"))
            }
        }
    };
}

secret! {
    /// The Site ID half of a Track API credential (Settings → API Credentials → Track API Keys).
    TrackSiteId
}
secret! {
    /// The API key half of a Track API credential (Settings → API Credentials → Track API Keys).
    TrackApiKey
}
secret! {
    /// An App API key, sent as a bearer token (Settings → API Credentials → App API Keys).
    AppApiKey
}
secret! {
    /// A Data Pipelines source API key (Data & Integrations → Sources → your HTTP API source).
    PipelinesApiKey
}

/// Holds the region and HTTP client, and hands out a ready-to-use `Configuration` for each
/// Customer.io API.
///
/// Customer.io's three APIs use three different credentials, so each accessor takes the
/// credential for its API: [`track`](CustomerIoClient::track) (Site ID + API key, HTTP Basic),
/// [`app`](CustomerIoClient::app) (App API key, Bearer) and
/// [`pipelines`](CustomerIoClient::pipelines) (source API key, HTTP Basic). Each returns a
/// `Configuration` with the region's base URL, that credential, and the client's middleware
/// chain already set.
///
/// ```no_run
/// use autogen_customerio::{AppApiKey, CustomerIoClient, Region};
/// let client = CustomerIoClient::new(Region::Us);
/// # #[cfg(feature = "app")]
/// let app = client.app(&AppApiKey::new("your-app-api-key")); // pass to autogen_customerio::app::apis functions
/// ```
///
/// Use [`CustomerIoClient::builder`] to attach `reqwest_middleware` middleware, such as a
/// tracing or retry layer, to every request the client makes.
#[derive(Debug, Clone)]
pub struct CustomerIoClient {
    region: Region,
    http: reqwest_middleware::ClientWithMiddleware,
}

impl CustomerIoClient {
    /// Create a client for a region, with no middleware attached.
    pub fn new(region: Region) -> Self {
        Self::builder(region).build()
    }

    /// Start building a [`CustomerIoClient`] with middleware attached to its HTTP client.
    pub fn builder(region: Region) -> CustomerIoClientBuilder {
        CustomerIoClientBuilder {
            region,
            middleware: Vec::new(),
        }
    }

    /// The region this client targets.
    pub fn region(&self) -> Region {
        self.region
    }

    fn user_agent() -> Option<String> {
        Some(format!("autogen-customerio/{}", env!("CARGO_PKG_VERSION")))
    }
}

/// Builds a [`CustomerIoClient`] with a chain of `reqwest_middleware` middleware.
///
/// Middleware runs in the order it is added, wrapping every HTTP request the resulting client
/// makes across all Customer.io APIs.
pub struct CustomerIoClientBuilder {
    region: Region,
    middleware: Vec<Arc<dyn reqwest_middleware::Middleware>>,
}

impl CustomerIoClientBuilder {
    /// Append a middleware to the chain.
    pub fn with<M: reqwest_middleware::Middleware>(mut self, middleware: M) -> Self {
        self.middleware.push(Arc::new(middleware));
        self
    }

    /// Append a shared middleware to the chain.
    pub fn with_arc(mut self, middleware: Arc<dyn reqwest_middleware::Middleware>) -> Self {
        self.middleware.push(middleware);
        self
    }

    /// Build the [`CustomerIoClient`].
    pub fn build(self) -> CustomerIoClient {
        let mut builder = reqwest_middleware::ClientBuilder::new(reqwest::Client::new());
        for middleware in self.middleware {
            builder = builder.with_arc(middleware);
        }
        CustomerIoClient {
            region: self.region,
            http: builder.build(),
        }
    }
}

impl std::fmt::Debug for CustomerIoClientBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomerIoClientBuilder")
            .field("region", &self.region)
            .field("middleware_count", &self.middleware.len())
            .finish()
    }
}

#[cfg(feature = "track")]
impl CustomerIoClient {
    /// Configuration for the Track API, authenticated with HTTP Basic (`site_id:api_key`).
    pub fn track(
        &self,
        site_id: &TrackSiteId,
        api_key: &TrackApiKey,
    ) -> crate::track::apis::configuration::Configuration {
        crate::track::apis::configuration::Configuration {
            base_path: match self.region {
                Region::Us => "https://track.customer.io",
                Region::Eu => "https://track-eu.customer.io",
            }
            .to_string(),
            user_agent: Self::user_agent(),
            client: self.http.clone(),
            basic_auth: Some((site_id.0.clone(), Some(api_key.0.clone()))),
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}

#[cfg(feature = "app")]
impl CustomerIoClient {
    /// Configuration for the App API, authenticated with `Authorization: Bearer <app-api-key>`.
    pub fn app(&self, api_key: &AppApiKey) -> crate::app::apis::configuration::Configuration {
        crate::app::apis::configuration::Configuration {
            base_path: match self.region {
                Region::Us => "https://api.customer.io",
                Region::Eu => "https://api-eu.customer.io",
            }
            .to_string(),
            user_agent: Self::user_agent(),
            client: self.http.clone(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: Some(api_key.0.clone()),
            api_key: None,
        }
    }
}

#[cfg(feature = "pipelines")]
impl CustomerIoClient {
    /// Configuration for the Data Pipelines API, authenticated with HTTP Basic (`api_key:`,
    /// empty password).
    pub fn pipelines(
        &self,
        api_key: &PipelinesApiKey,
    ) -> crate::pipelines::apis::configuration::Configuration {
        crate::pipelines::apis::configuration::Configuration {
            base_path: match self.region {
                Region::Us => "https://cdp.customer.io/v1",
                Region::Eu => "https://cdp-eu.customer.io/v1",
            }
            .to_string(),
            user_agent: Self::user_agent(),
            client: self.http.clone(),
            basic_auth: Some((api_key.0.clone(), None)),
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}
