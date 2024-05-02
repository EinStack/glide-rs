#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
//! ## Usage
//!
//! ```rust,no_run
//! use glide_rs::{Client, Result};
//!
//! async fn main() -> Result<()> {
//!     let glide = Client::default();
//!     glide.health().await?;
//!     let _ = glide.language.list().await?;
//!     Ok(())
//! }
//! ```

pub use client::Client;
pub use service::language;

mod client;
mod config;
mod service;

/// Errors that may occur during the processing of API request.
#[derive(Debug, thiserror::Error, serde::Deserialize)]
#[error("{message}")]
pub struct SvcError {
    #[serde(skip)]
    pub status_code: reqwest::StatusCode,
    // TODO: Empty if Option<String> is None.
    pub message: String,
}

/// Error type for operations of a [`Client`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors that may occur during the processing an HTTP request.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Errors that may occur during the processing an WS request.
    #[error("websocket error: {0}")]
    Ws(#[from] reqwest_websocket::Error),

    /// Errors that may occur during the processing of API request.
    #[error("glide error: {0}")]
    Glide(#[from] SvcError),
}

/// Specialized [`Result`] type for an [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
