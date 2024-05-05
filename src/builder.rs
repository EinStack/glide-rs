use std::fmt;

use reqwest::{Client as RwClient, Url};

use crate::{Client, Config};

/// [`Client`] builder.
#[must_use]
pub struct Builder {
    api_key: String,
    base_url: Option<Url>,
    user_agent: Option<String>,
    http_client: Option<RwClient>,
}

impl Builder {
    /// Creates a new [`Builder`].
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_owned(),
            base_url: None,
            user_agent: None,
            http_client: None,
        }
    }

    /// Overrides the `base URL`.
    ///
    /// Default value: <api.einstack.com>
    pub fn with_base_url(mut self, base_url: Url) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Overrides the `User-Agent` header.
    ///
    /// Default value: `glide-rs/0.1.0`
    pub fn with_user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_owned());
        self
    }

    /// Overrides the `HTTP` client.
    ///
    /// Default value: `reqwest::Client::default()`
    pub fn with_http_client(mut self, client: RwClient) -> Self {
        self.http_client = Some(client);
        self
    }

    /// Creates a new [`Client`].
    pub fn build(self) -> Client {
        let config = Config {
            api_key: self.api_key,
            user_agent: self.user_agent.unwrap_or_else(default_user_agent),
            base_url: self.base_url.unwrap_or_else(default_base_url),
            client: self.http_client.unwrap_or_default(),
        };

        config.into_client()
    }
}

impl fmt::Debug for Builder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Builder")
            .field("user_agent", &self.user_agent.is_some())
            .field("base_url", &self.base_url.is_some())
            .field("http_client", &self.http_client.is_some())
            .finish_non_exhaustive()
    }
}

fn default_base_url() -> Url {
    Url::parse("https://api.einstack.com").unwrap()
}

fn default_user_agent() -> String {
    format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}
