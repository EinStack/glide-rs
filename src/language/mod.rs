//! Language service, its request and response types.
//!

use std::fmt;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, ready};

use futures::{Sink, SinkExt, Stream, StreamExt};
use reqwest::Method;
use reqwest_websocket::{Message, RequestBuilderExt, WebSocket};
use serde_json::Value;

use crate::{Error, Result};
use crate::config::Config;
use crate::language::types::{ChatRequest, ChatResponse, RouterConfigs};

pub mod types;

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
    /// [`StatusCode`]: reqwest::StatusCode
    pub async fn stream(&self, router: &str) -> Result<Chat> {
        let path = format!("/v1/language/{router}/chatStream");

        let request = self.0.create(Method::GET, &path).upgrade();
        let response = request.send().await?;
        let websocket = response.into_websocket().await?;

        Ok(Chat { inner: websocket })
    }
}

impl fmt::Debug for Language {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// Streaming (`WebSocket`) chat connection.
///
/// Implements `futures::`[`Stream`] and `futures::`[`Sink`].
#[must_use = "streams do nothing unless you poll them"]
pub struct Chat {
    inner: WebSocket,
}

impl Stream for Chat {
    type Item = Result<Value>;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let poll = ready!(self.inner.poll_next_unpin(cx));
        let next = poll.map(|x| x.and_then(|x| x.json()).map_err(Into::into));

        Poll::Ready(next)
    }
}

impl Sink<Value> for Chat {
    type Error = Error;

    #[inline]
    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready_unpin(cx).map_err(Into::into)
    }

    #[inline]
    fn start_send(mut self: Pin<&mut Self>, item: Value) -> Result<(), Self::Error> {
        let item = Message::text_from_json(&item)?;
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

    use crate::{Client, Result};
    use crate::language::types::ChatRequest;

    #[tokio::test]
    async fn list() -> Result<()> {
        let glide = Client::default();
        let response = glide.lang.list().await?;
        assert_eq!(response.len(), 1);
        dbg!(&response[0]);

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
    async fn stream() -> Result<()> {
        let glide = Client::default();

        let router = "myrouter";
        let ws = glide.lang.stream(router).await?;
        let (tx, rx) = ws.split();
        // TODO: Test streaming chat.

        Ok(())
    }
}
