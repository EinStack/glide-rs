//! Language service, its request and response types.
//!

use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::config::Config;
use crate::lang::chat::{ChatRequest, ChatResponse};
use crate::lang::list::RouterConfigs;
#[cfg(feature = "streaming")]
#[cfg_attr(docsrs, doc(cfg(feature = "streaming")))]
pub use crate::lang::stream::Chat;
use crate::Result;

pub mod chat;
pub mod list;

#[cfg(feature = "streaming")]
mod stream;

/// APIs for `/v1/language` endpoints.
#[derive(Clone)]
pub struct Language(pub(crate) Arc<Config>);

impl Language {
    /// Retrieves a list of all `router` configs.
    ///
    /// `GET /v1/language`
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the response [`StatusCode`] is not in the 200-299 range.
    ///
    /// [`Error`]: crate::Error
    /// [`StatusCode`]: reqwest::StatusCode
    pub async fn list(&self) -> Result<RouterConfigs> {
        let request = self.0.create(Method::GET, "/v1/language/");
        let response = self.0.send(request).await?;
        let content = response.json::<RouterConfigs>().await?;
        Ok(content)
    }

    /// Sends a single chat request to a specified `router` and retrieves the response.
    ///
    /// `POST /v1/language/{router}/chat`
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the response [`StatusCode`] is not in the 200-299 range.
    ///
    /// [`Error`]: crate::Error
    /// [`StatusCode`]: reqwest::StatusCode
    pub async fn chat(&self, router: &str, data: ChatRequest) -> Result<ChatResponse> {
        let path = format!("/v1/language/{router}/chat");

        let request = self.0.create(Method::POST, &path);
        let response = self.0.send(request.json(&data)).await?;
        let content = response.json::<ChatResponse>().await?;

        Ok(content)
    }

    /// Establishes a `WebSocket` connection for streaming chat messages from a specified `router`.
    ///
    /// `GET /v1/language/{router}/chatStream`
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the response [`StatusCode`] is not in the 200-299 range.
    ///
    /// [`Error`]: crate::Error
    /// [`StatusCode`]: reqwest::StatusCode
    #[cfg(feature = "streaming")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streaming")))]
    pub async fn stream(&self, router: &str) -> Result<Chat> {
        use reqwest_websocket::RequestBuilderExt as _;
        let path = format!("/v1/language/{router}/chatStream");

        let request = self.0.create(Method::GET, &path).upgrade();
        let response = request.send().await?;
        let websocket = response.into_websocket().await?;

        Ok(Chat::new(websocket))
    }
}

impl fmt::Debug for Language {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod test {
    use crate::lang::chat::ChatRequest;
    use crate::{Client, Result};

    #[tokio::test]
    async fn list() -> Result<()> {
        let glide = Client::default();
        let response = glide.lang.list().await?;
        assert!(response.routers.len() > 0);

        Ok(())
    }

    #[tokio::test]
    async fn chat() -> Result<()> {
        let glide = Client::default();

        let router = "myrouter";
        let request = ChatRequest::new("Hello!");
        let _ = glide.lang.chat(router, request).await?;

        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "streaming")]
    async fn stream() -> Result<()> {
        let glide = Client::default();

        let router = "myrouter";
        let _ws = glide.lang.stream(router).await?;
        // let (tx, rx) = ws.split();
        // TODO: Test streaming chat.

        Ok(())
    }
}
