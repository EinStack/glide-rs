use std::pin::Pin;
use std::task::{ready, Context, Poll};

use futures::{Sink, SinkExt, Stream, StreamExt};
use reqwest_websocket::{CloseCode, Message, WebSocket};
use serde_json::Value;

use crate::{Error, Result};

/// Streaming (`WebSocket`) chat connection.
///
/// Implements `futures::`[`Stream`] and `futures::`[`Sink`].
#[must_use = "streams do nothing unless you poll them"]
pub struct Chat {
    inner: WebSocket,
}

impl Chat {
    /// Creates a new [`Chat`] connection.
    #[inline]
    pub(crate) fn new(inner: WebSocket) -> Self {
        Self { inner }
    }

    /// Closes the underlying connection after sending [`CloseCode::Away`].
    pub async fn close(self) -> Result<()> {
        let response = self.inner.close(CloseCode::Away, None).await;
        response.map_err(Into::into)
    }
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
