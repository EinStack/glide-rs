//! <https://anthropic.com/>

use serde::{Deserialize, Serialize};

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicConfig {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "chatEndpoint")]
    pub chat_endpoint: String,
    #[serde(rename = "defaultParams", skip_serializing_if = "Option::is_none")]
    pub default_params: Option<AnthropicParams>,
    #[serde(rename = "model")]
    pub model: String,
}

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
}
