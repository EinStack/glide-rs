use serde::{Deserialize, Serialize};

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct CohereConfig {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "chatEndpoint")]
    pub chat_endpoint: String,
    #[serde(rename = "defaultParams", skip_serializing_if = "Option::is_none")]
    pub default_params: Option<CohereParams>,
    /// <https://docs.cohere.com/docs/models#command>
    #[serde(rename = "model")]
    pub model: String,
}

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct CohereParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preamble: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_truncation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_queries_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    pub temperature: f64,
}
