//! TODO.

use serde::{Deserialize, Serialize};

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct BedrockConfig {
    #[serde(rename = "awsRegion")]
    pub aws_region: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "chatEndpoint")]
    pub chat_endpoint: String,
    #[serde(rename = "defaultParams", skip_serializing_if = "Option::is_none")]
    pub default_params: Option<BedrockParams>,
    #[serde(rename = "model")]
    pub model: String,
}

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct BedrockParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
}
