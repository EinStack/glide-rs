#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use builder::Builder;
pub use client::Client;
pub(crate) use config::Config;
pub use service::language;

mod builder;
mod client;
mod config;
mod service;

pub mod types {
    //! Request and response types.

    /// Errors that may occur during the processing of API request.
    #[derive(Debug, thiserror::Error, serde::Deserialize)]
    #[error("{message}")]
    pub struct ErrorResponse {
        #[serde(skip)]
        pub status_code: u16,
        pub message: String,
    }
}

/// Error type for a [`Client`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors that may occur during the processing of an HTTP request.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Errors that may occur during the processing of a WS request.
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
