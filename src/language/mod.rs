//! Language service, its request and response types.
//!

use std::fmt;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures::{Sink, SinkExt, Stream, StreamExt};
use reqwest::Method;
use reqwest_websocket::{Message, RequestBuilderExt, WebSocket};

use crate::config::Config;
use crate::language::types::{ChatRequest, ChatResponse, RouterConfig};
use crate::{Error, Result};

pub mod types;

/// APIs for `/v1/language` endpoints.
#[derive(Clone)]
pub struct LanguageSvc(pub(crate) Arc<Config>);

impl LanguageSvc {
    /// Retrieves a list of all router configs.
    ///
    /// `GET /v1/language`
    pub async fn list(&self) -> Result<Vec<RouterConfig>> {
        #[derive(Debug, serde::Deserialize)]
        struct RouterConfigs {
            pub routers: Vec<RouterConfig>,
        }

        let request = self.0.create(Method::GET, "/v1/language/");
        let response = self.0.send(request).await?;
        let content = response.json::<RouterConfigs>().await?;
        Ok(content.routers)
    }

    /// Sends a single chat request to a specified router and retrieves the response.
    ///
    /// `POST /v1/language/{router}/chat`
    pub async fn chat(&self, router: &str, data: ChatRequest) -> Result<ChatResponse> {
        let path = format!("/v1/language/{router}/chat");

        let request = self.0.create(Method::POST, &path);
        let response = self.0.send(request.json(&data)).await?;
        let content = response.json::<ChatResponse>().await?;

        Ok(content)
    }

    /// Establishes a WebSocket connection for streaming chat messages from a specified router.
    ///
    /// `GET /v1/language/{router}/chatStream`
    pub async fn stream(&self, router: &str) -> Result<Chat> {
        let path = format!("/v1/language/{router}/chatStream");

        let request = self.0.create(Method::GET, &path).upgrade();
        let response = request.send().await?;
        let websocket = response.into_websocket().await?;

        Ok(Chat { inner: websocket })
    }
}

impl fmt::Debug for LanguageSvc {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// Streaming chat WebSocket connection.
///
/// Implements `futures::`[`Stream`] and `futures::`[`Sink`].
pub struct Chat {
    inner: WebSocket,
}

impl Stream for Chat {
    // TODO: poll::map_ok
    type Item = Result<Message>;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.poll_next_unpin(cx).map_err(Into::into)
    }
}

impl Sink<Message> for Chat {
    // TODO: serde_json::into_string
    type Error = Error;

    #[inline]
    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready_unpin(cx).map_err(Into::into)
    }

    #[inline]
    fn start_send(mut self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        self.inner.start_send_unpin(item).map_err(Into::into)
    }

    #[inline]
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_flush_unpin(cx).map_err(Into::into)
    }

    #[inline]
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_close_unpin(cx).map_err(Into::into)
    }
}

#[cfg(test)]
mod test {
    use futures::StreamExt;

    use crate::language::types::ChatRequest;
    use crate::{Client, Result};

    #[tokio::test]
    async fn list() -> Result<()> {
        let glide = Client::default();
        let response = glide.language.list().await?;
        assert_eq!(response.len(), 1);
        dbg!(&response[0]);

        Ok(())
    }

    #[tokio::test]
    async fn chat() -> Result<()> {
        let glide = Client::default();

        let router = "myrouter";
        let request = ChatRequest::new("Hello!");
        let _ = glide.language.chat(router, request).await?;

        Ok(())
    }

    #[tokio::test]
    async fn stream() -> Result<()> {
        let glide = Client::default();

        let router = "myrouter";
        let ws = glide.language.stream(router).await?;
        let (tx, rx) = ws.split();
        // TODO: test streaming chat.

        Ok(())
    }
}
