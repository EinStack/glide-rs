//! Request and response types for `/v1/language/list` endpoints.
//!

use serde::Deserialize;
use serde_json::Value;

/// All router configurations.
#[derive(Debug, Deserialize)]
pub struct RouterConfigs {
    /// List of all available routers.
    pub routers: Vec<Value>,
}
