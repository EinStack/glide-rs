#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use client::Client;
pub use service::language;

mod client;
mod config;
pub mod provider;
mod service;

/// Errors that may occur during the processing of API request.
#[derive(Debug, thiserror::Error, serde::Deserialize)]
#[error("resend error: {message}")]
pub struct SvcError {
    pub message: String,
}

/// Error type for operations of a [`Client`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors that may occur during the processing an HTTP request.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Errors that may occur during the processing of API request.
    #[error("glide error: {0}")]
    Glide(#[from] SvcError),
}

/// Specialized [`Result`] type for an [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
