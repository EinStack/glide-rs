#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
//!
//! ```rust
//! use glide_rs::Client;
//!
//! let client = Client::new();
//! ```
//!

pub use client::Client;

mod client;
mod providers;
mod config;

pub mod services {
    //! `EinStack` API services.

    pub use super::providers::Anthropic;
}

pub mod types {
    //! Request and response types.


}

/// Error type for operations of a [`Client`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors that may occur during the processing an HTTP request.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Errors that may occur during the processing of the API request.
    #[error("glide error: {0}")]
    Glide(#[from] types::GlideError),
}

/// Specialized [`Result`] type for an [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
