use std::fmt;
use std::sync::Arc;

use reqwest::{Client as ReqwestClient, Method};

use crate::config::Config;
use crate::language::LanguageSvc;
use crate::Result;

/// A minimal [EinStack](https://einstack.ai/) client.
///
/// #### Example
///
/// ```rust,no_run
/// use glide_rs::Client;
///
/// # let _ = async {
/// let glide = Client::default();
/// glide.health().await?;
/// let _ = glide.language.list().await?;
/// # };
/// ```
#[must_use]
#[derive(Clone)]
pub struct Client {
    config: Arc<Config>,
    pub language: LanguageSvc,
}

impl Client {
    /// Creates a new [`EinStack`] client.
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `GLIDE_BASE_URL` is set but is not a valid `URL`.
    ///
    /// [`EinStack`]: https://www.einstack.ai/
    pub fn new() -> Self {
        let client = ReqwestClient::new();
        Self::with_client(client)
    }

    /// Creates a new [`EinStack`] client with a provided [`reqwest::Client`].
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `GLIDE_BASE_URL` is set but is not a valid `URL`.
    ///
    /// [`EinStack`]: https://www.einstack.ai/
    /// [`reqwest::Client`]: ReqwestClient
    pub fn with_client(client: ReqwestClient) -> Self {
        let config = Arc::new(Config::new(client));
        Self {
            language: LanguageSvc(config.clone()),
            config,
        }
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
    fn default() -> Self {
        Self::new()
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
