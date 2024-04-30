use serde::{Deserialize, Serialize};

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiConfig {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "chatEndpoint")]
    pub chat_endpoint: String,
    #[serde(rename = "defaultParams", skip_serializing_if = "Option::is_none")]
    pub default_params: Option<OpenAiParams>,
    #[serde(rename = "model")]
    pub model: String,
}

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<std::collections::HashMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
