use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::config::Config;
use crate::Result;

/// TODO.
#[derive(Clone)]
pub struct Language(pub(crate) Arc<Config>);

impl Language {
    /// TODO.
    pub async fn list(&self) -> Result<Vec<types::RouterConfig>> {
        let request = self.0.build(Method::GET, "/v1/language/");
        let response = self.0.send(request).await?;
        let content = response.json::<types::RouterConfigs>().await?;
        Ok(content.routers)
    }

    /// TODO.
    pub async fn chat(&self) -> Result<()> { todo!() }

    /// TODO.
    pub async fn stream(&self) -> Result<()> { todo!() }
}

impl fmt::Debug for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
    pub struct RouterConfigs {
        pub routers: Vec<RouterConfig>,
    }

    /// TODO.
    #[derive(Debug, Deserialize)]
    pub struct RouterConfig {
        /// Is router enabled.
        pub enabled: bool,
        /// The list of models that could handle requests.
        pub models: Vec<LangModelConfig>,
        /// Retry when no healthy model is available to router.
        pub retry: RetryConfig,
        /// Unique router ID.
        pub routers: String,
        /// Strategy on picking the next model to serve the request.
        pub strategy: String,
    }

    /// TODO.
    #[derive(Debug, Deserialize)]
    pub struct RetryConfig {
        pub base_multiplier: Option<i32>,
        pub max_delay: Option<i32>,
        pub max_retries: Option<i32>,
        pub min_delay: Option<i32>,
    }

    /// TODO.
    #[derive(Debug, Deserialize)]
    pub struct LangModelConfig {
        /// Model instance ID (unique in scope of the router).
        pub id: String,
        /// Is the model enabled?
        pub enabled: bool,
        pub weight: Option<i32>,
        pub error_budget: Option<String>,

        pub anthropic: Option<AnthropicConfig>,
        // pub azureopenai: Option<Box<models::AzureopenaiPeriodConfig>>,
        // pub bedrock: Option<Box<models::BedrockPeriodConfig>>,
        // pub client: Option<Box<models::ClientsPeriodClientConfig>>,
        // pub cohere: Option<Box<models::CoherePeriodConfig>>,
        // pub latency: Option<Box<models::LatencyPeriodConfig>>,
        // pub octoml: Option<Box<models::OctomlPeriodConfig>>,
        pub ollama: Option<OllamaConfig>,
        pub openai: Option<OpenAiConfig>,
    }

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
}

#[cfg(test)]
mod test {
    use crate::{Client, Result};

    #[tokio::test]
    async fn list() -> Result<()> {
        let glide = Client::default();
        let _ = glide.language.list().await?;
        Ok(())
    }

    #[tokio::test]
    async fn chat() -> Result<()> {
        let glide = Client::default();
        // TODO.
        Ok(())
    }

    #[tokio::test]
    async fn stream() -> Result<()> {
        let glide = Client::default();
        // TODO.
        Ok(())
    }
}
