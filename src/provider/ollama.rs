//! <https://ollama.com/>

use serde::{Deserialize, Serialize};

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaConfig {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "chatEndpoint")]
    pub chat_endpoint: String,
    #[serde(rename = "defaultParams", skip_serializing_if = "Option::is_none")]
    pub default_params: Option<OllamaParams>,
    #[serde(rename = "model")]
    pub model: String,
}

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microstat: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microstat_eta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microstat_tau: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_ctx: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_gpu: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_gqa: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_predict: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_thread: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_last_n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfs_z: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
}
