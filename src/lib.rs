#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use builder::Builder;
pub use client::Client;
pub(crate) use config::Config;

mod builder;
mod client;
mod config;
pub mod language;

pub mod types {
    //! Request and response types.

    /// List specifying general categories of [`ErrorResponse`]s.
    #[derive(Debug, Copy, Clone)]
    pub enum ErrorKind {
        /// Error name is not a part of the implemented `API` spec.
        Unrecognized,

        UnsupportedMediaType,
        RouteNotFound,
        PayloadParseError,
        RouterNotFound,
        NoModelConfigured,
        ModelUnavailable,
        AllModelsUnavailable,
        UnknownError,
    }

    /// Errors that may occur during the processing of API request.
    #[derive(Debug, thiserror::Error, serde::Deserialize)]
    #[error("{message}")]
    pub struct ErrorResponse {
        pub name: String,
        pub message: String,

        #[serde(skip)]
        pub status_code: u16,
    }

    impl ErrorResponse {
        /// Returns the [`ErrorKind`].
        pub fn kind(&self) -> ErrorKind {
            match self.name.as_str() {
                "unsupported_media_type" => ErrorKind::UnsupportedMediaType,
                "route_not_found" => ErrorKind::RouteNotFound,
                "payload_parse_error" => ErrorKind::PayloadParseError,
                "router_not_found" => ErrorKind::RouterNotFound,
                "no_model_configured" => ErrorKind::NoModelConfigured,
                "model_unavailable" => ErrorKind::ModelUnavailable,
                "all_models_unavailable" => ErrorKind::AllModelsUnavailable,
                "unknown_error" => ErrorKind::UnknownError,
                _ => ErrorKind::Unrecognized,
            }
        }
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
