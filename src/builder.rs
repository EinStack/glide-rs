use std::{env, fmt};

use reqwest::{Client as RwClient, Url};

use crate::{Client, Config};

/// [`Client`] builder.
#[must_use]
#[derive(Clone)]
pub struct Builder {
    api_key: Option<String>,
    base_url: Option<Url>,
    user_agent: Option<String>,
    http_client: Option<RwClient>,
}

impl Builder {
    /// Creates a new [`Builder`].
    ///
    /// Same as [`Client::builder`].
    pub const fn new() -> Self {
        Self {
            api_key: None,
            base_url: None,
            user_agent: None,
            http_client: None,
        }
    }

    /// Attaches the `API key`.
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_owned());
        self
    }

    /// Overrides the `base URL`.
    ///
    /// Default value: <http://127.0.0.1:9099/>
    pub fn with_base_url(mut self, base_url: Url) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Overrides the `User-Agent` header.
    ///
    /// Default value: `Glide/0.1.0 (Rust; Ver 1.70.0)`
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
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `GLIDE_API_KEY` is set but is not a valid `String`.
    /// - Panics if the environment variable `GLIDE_BASE_URL` is set but is not a valid `URL`.
    /// - Panics if the environment variable `GLIDE_USER_AGENT` is set but is not a valid `String`.
    pub fn build(self) -> Client {
        let config = Config {
            api_key: self.api_key.or_else(default_api_key),
            user_agent: self.user_agent.unwrap_or_else(default_user_agent),
            base_url: self.base_url.unwrap_or_else(default_base_url),
            client: self.http_client.unwrap_or_default(),
        };

        config.into_client()
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
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

fn default_api_key() -> Option<String> {
    match env::var("GLIDE_API_KEY") {
        Ok(var) => Some(var),
        Err(env::VarError::NotPresent) => None,
        Err(env::VarError::NotUnicode(_)) => {
            panic!("env variable `GLIDE_BASE_URL` should be a valid `String`")
        }
    }
}

fn default_base_url() -> Url {
    env::var("GLIDE_BASE_URL")
        .map_or_else(|_| Url::parse("http://127.0.0.1:9099/"), |x| Url::parse(&x))
        .expect("env variable `GLIDE_BASE_URL` should be a valid `URL`")
}

fn default_user_agent() -> String {
    if let Ok(x) = env::var("GLIDE_USER_AGENT") {
        return x;
    };

    format!(
        "Glide/{} (Rust; Ver {})",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_RUST_VERSION")
    )
}
