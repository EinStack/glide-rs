#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub use builder::Builder;
pub use client::Client;
pub(crate) use config::Config;

mod builder;
mod client;
mod config;
mod error;
pub mod lang;

pub mod types {
    //! Request and response types.
    //!

    pub use super::error::{ErrorKind, ErrorResponse};
}

/// Error type for a [`Client`].
#[must_use]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors that may occur during the processing of an HTTP request.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Errors that may occur during the processing of a WS request.
    #[cfg(feature = "streaming")]
    #[cfg_attr(docsrs, doc(cfg(feature = "streaming")))]
    #[error("websocket error: {0}")]
    Ws(#[from] reqwest_websocket::Error),

    /// Errors that may occur during the processing of an API request.
    #[error("api error: {0}")]
    Api(#[from] types::ErrorResponse),
}

/// Specialized [`Result`] type for an [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
