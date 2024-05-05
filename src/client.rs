use std::sync::Arc;
use std::{env, fmt};

use reqwest::{Client as RwClient, Method, Url};

use crate::language::LanguageSvc;
use crate::{Builder, Config, Result};

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
    pub(crate) config: Arc<Config>,
    /// `Glide` APIs for `/v1/language` endpoints.
    pub language: LanguageSvc,
}

impl Client {
    /// Creates a new [`EinStack`] `Glide` client.
    ///
    /// [`EinStack`]: https://www.einstack.ai/
    pub fn new(api_key: &str) -> Self {
        Builder::new(api_key).build()
    }

    /// Creates a new [`Client`] builder.
    pub fn builder(api_key: &str) -> Builder {
        Builder::new(api_key)
    }

    /// Returns the reference to the provided `API key`.
    #[inline]
    #[must_use]
    pub fn api_key(&self) -> &str {
        self.config.api_key.as_ref()
    }

    /// Returns the reference to the used `User-Agent` header value.
    #[inline]
    #[must_use]
    pub fn user_agent(&self) -> &str {
        self.config.user_agent.as_str()
    }

    /// Returns the reference to the used base `URL`.
    #[inline]
    #[must_use]
    pub fn base_url(&self) -> &str {
        self.config.base_url.as_str()
    }

    /// Returns the underlying [`reqwest::Client`].
    ///
    /// [`reqwest::Client`]: RwClient
    #[inline]
    #[must_use]
    pub fn client(&self) -> &RwClient {
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

        let request = self.config.create(Method::GET, "/v1/health/");
        let response = self.config.send(request).await?;
        let content = response.json::<Health>().await?;

        Ok(content.healthy)
    }
}

impl Default for Client {
    /// Creates a new [`Client`] from environment variables.
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `GLIDE_API_KEY` is not set.
    /// - Panics if the environment variable `GLIDE_BASE_URL` is set but is not a valid `URL`.
    /// - Panics if the environment variable `GLIDE_USER_AGENT` is set but is not a valid `String`.
    fn default() -> Self {
        let api_key = env::var("GLIDE_API_KEY")
            .expect("env variable `GLIDE_API_KEY` should be a valid API key");
        let mut builder = Self::builder(&api_key);

        if let Ok(x) = env::var("GLIDE_BASE_URL") {
            builder = builder.with_base_url(
                Url::parse(&x).expect("env variable `GLIDE_BASE_URL` should be a valid URL"),
            );
        }

        if let Ok(x) = env::var("GLIDE_USER_AGENT") {
            builder = builder.with_user_agent(&x);
        }

        builder.build()
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.config, f)
    }
}

#[cfg(test)]
mod test {
    use crate::{Client, Result};

    #[test]
    fn create() -> Result<()> {
        let _ = Client::new("");
        let _ = Client::builder("").build();
        let _ = Client::default();
        Ok(())
    }

    #[tokio::test]
    async fn health() -> Result<()> {
        let glide = Client::default();
        assert!(glide.health().await.is_ok());
        Ok(())
    }
}
