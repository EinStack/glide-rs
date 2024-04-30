use std::{env, fmt};

use reqwest::header::USER_AGENT;
use reqwest::{Client, Method, RequestBuilder, Response, Url};

use crate::{Error, Result, SvcError};

pub struct Config {
    pub(crate) api_key: String,
    pub(crate) user_agent: String,
    pub(crate) base_url: Url,
    pub(crate) client: Client,
}

impl Config {
    /// Creates a new [`Config`].
    pub fn new(api_key: &str, client: Client) -> Self {
        let env_base_url = env::var("GLIDE_BASE_URL")
            .map_or_else(
                |_| Url::parse("http://127.0.0.1:9099"),
                |env_var| Url::parse(env_var.as_str()),
            )
            .expect("env variable `GLIDE_BASE_URL` should be a valid URL");

        let env_user_agent = env::var("GLIDE_USER_AGENT").unwrap_or_else(|_| {
            format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        });

        Self {
            api_key: api_key.to_owned(),
            user_agent: env_user_agent,
            base_url: env_base_url,
            client,
        }
    }

    /// Constructs a new [`RequestBuilder`].
    pub fn build(&self, method: Method, path: &str) -> RequestBuilder {
        let path = self
            .base_url
            .join(path)
            .expect("should be a valid API endpoint");

        self.client
            .request(method, path)
            .bearer_auth(self.api_key.as_str())
            .header(USER_AGENT, self.user_agent.as_str())
    }

    /// Builds and executes the given [`RequestBuilder`].
    pub async fn send(&self, request: RequestBuilder) -> Result<Response> {
        let request = request.build()?;
        let response = self.client.execute(request).await?;

        match response.status() {
            x if x.is_client_error() || x.is_server_error() => {
                let error = response.json::<SvcError>().await?;
                Err(Error::Glide(error))
            }
            _ => Ok(response),
        }
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client").finish_non_exhaustive()
    }
}
