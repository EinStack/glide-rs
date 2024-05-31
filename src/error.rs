use serde::Deserialize;
use thiserror::Error;

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
#[derive(Debug, Error, Deserialize)]
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
