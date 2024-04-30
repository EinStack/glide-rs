#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use client::Client;
pub use service::language;

mod client;
mod config;
mod service;
pub mod provider;

pub mod types {
    //! Request and response types.
}

/// Errors that may occur during the processing of the API request.
#[derive(Debug, thiserror::Error)]
#[error("unknown")]
pub struct ErrorResponse {}

/// Error type for operations of a [`Client`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors that may occur during the processing an HTTP request.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Errors that may occur during the processing of the API request.
    #[error("glide error: {0}")]
    Glide(#[from] ErrorResponse),
}

/// Specialized [`Result`] type for an [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
