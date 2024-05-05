use std::fmt;
use std::sync::Arc;

use reqwest::header::USER_AGENT;
use reqwest::{Client as RwClient, Method, RequestBuilder, Response, Url};

use crate::language::LanguageSvc;
use crate::{types::ErrorResponse, Client, Error, Result};

pub struct Config {
    pub user_agent: String,
    pub api_key: String,
    pub base_url: Url,
    pub client: RwClient,
}

impl Config {
    /// Creates a new [`RequestBuilder`].
    pub fn create(&self, method: Method, path: &str) -> RequestBuilder {
        let path = self
            .base_url
            .join(path)
            .expect("should be a valid `API` endpoint");

        self.client
            .request(method, path)
            .bearer_auth(&self.api_key)
            .header(USER_AGENT, &self.user_agent)
    }

    /// Builds and executes the [`RequestBuilder`].
    pub async fn send(&self, request_builder: RequestBuilder) -> Result<Response> {
        let request = request_builder.build()?;
        let response = self.client.execute(request).await?;

        match response.status() {
            x if x.is_client_error() || x.is_server_error() => {
                let error = response.json::<ErrorResponse>().await?;
                Err(Error::Api(error))
            }
            _ => Ok(response),
        }
    }

    /// Creates a new [`Client`].
    pub fn into_client(self) -> Client {
        let config = Arc::new(self);

        Client {
            config: config.clone(),
            language: LanguageSvc(config),
        }
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("api_key", &"*********")
            .field("user_agent", &self.user_agent.as_str())
            .field("base_url", &self.base_url.as_str())
            .finish_non_exhaustive()
    }
}
