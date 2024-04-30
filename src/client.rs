use std::{env, fmt};
use std::sync::Arc;

use reqwest::{Client as ReqwestClient, Method};

use crate::config::Config;
use crate::language::Language;
use crate::Result;

/// A minimal [EinStack](https://einstack.ai/) client.
///
/// #### Example
///
/// ```rust,no_run
/// use glide_rs::Client;
///
/// let client = Client::new("");
/// ```
#[derive(Clone)]
pub struct Client {
    config: Arc<Config>,
    pub language: Language,
}

impl Client {
    /// Creates a new [`EinStack`] client.
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `GLIDE_BASE_URL` is set but is not a valid `URL`.
    ///
    /// [`EinStack`]: https://www.einstack.ai/
    pub fn new(api_key: &str) -> Self {
        let client = ReqwestClient::new();
        Self::with_client(api_key, client)
    }

    /// Creates a new [`EinStack`] client with a provided [`reqwest::Client`].
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `GLIDE_BASE_URL` is set but is not a valid `URL`.
    ///
    /// [`EinStack`]: https://www.einstack.ai/
    /// [`reqwest::Client`]: ReqwestClient
    pub fn with_client(api_key: &str, client: ReqwestClient) -> Self {
        let config = Arc::new(Config::new(api_key, client));
        Self { language: Language(config.clone()), config }
    }

    /// Returns the reference to the provided API key.
    #[inline]
    #[must_use]
    pub fn api_key(&self) -> &str {
        self.config.api_key.as_ref()
    }

    /// Returns the reference to the used `User-Agent` header value.
    ///
    /// ### Notes
    ///
    /// Use the `GLIDE_USER_AGENT` environment variable to override.
    #[inline]
    #[must_use]
    pub fn user_agent(&self) -> &str {
        self.config.user_agent.as_str()
    }

    /// Returns the reference to the used base `URL`.
    ///
    /// ### Notes
    ///
    /// Use the `GLIDE_BASE_URL` environment variable to override.
    #[inline]
    #[must_use]
    pub fn base_url(&self) -> &str {
        self.config.base_url.as_str()
    }

    /// Returns the underlying [`reqwest::Client`].
    ///
    /// [`reqwest::Client`]: ReqwestClient
    #[inline]
    #[must_use]
    pub fn client(&self) -> &ReqwestClient {
        &self.config.client
    }
}

impl Client {
    /// Returns `true` if the service is healthy.
    ///
    /// `GET /v1/health`
    pub async fn health(&self) -> Result<bool> {
        #[derive(Debug, serde::Deserialize)]
        pub struct Health {
            pub healthy: bool,
        }

        let request = self.config.build(Method::GET, "/v1/health/");
        let response = self.config.send(request).await?;
        let content = response.json::<Health>().await?;

        Ok(content.healthy)
    }
}

impl Default for Client {
    /// Creates a new [`Client`] from the `GLIDE_API_KEY` environment variable .
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `GLIDE_API_KEY` is not set.
    /// - Panics if the environment variable `GLIDE_BASE_URL` is set but is not a valid `URL`.
    fn default() -> Self {
        let api_key = env::var("GLIDE_API_KEY")
            .expect("env variable `GLIDE_API_KEY` should be a valid API key");

        Self::new(api_key.as_str())
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.config, f)
    }
}

mod types {}

#[cfg(test)]
mod test {
    use crate::{Client, Result};

    #[tokio::test]
    async fn health() -> Result<()> {
        let glide = Client::default();
        assert!(glide.health().await.is_ok());

        Ok(())
    }
}
