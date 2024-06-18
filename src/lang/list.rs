//! Request and response types for `/v1/language/list` endpoints.
//!

use serde::{Deserialize, Serialize};

/// All router configurations.
#[derive(Debug, Deserialize)]
pub struct RouterConfigs {
    /// List of all available routers.
    pub routers: Vec<RouterConfig>,
}

/// Single router configuration.
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

/// Retry configuration.
#[derive(Debug, Deserialize)]
pub struct RetryConfig {
    pub base_multiplier: i32,
    pub max_delay: i64,
    pub min_delay: i64,
    pub max_retries: i32,
}

/// Provider model configuration.
#[derive(Debug, Deserialize)]
pub struct LangModelConfig {
    /// Model instance ID (unique in scope of the router).
    pub id: String,
    /// Is the model enabled?
    pub enabled: bool,
    pub weight: Option<i32>,
    pub error_budget: Option<String>,

    #[serde(flatten)]
    pub config: ProviderConfig,

    pub client: Option<ClientsConfig>,
    pub latency_config: Option<LatencyConfig>,
}

/// Timeout configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// Latency configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct LatencyConfig {
    /// Weight of new latency measurements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decay: Option<f64>,
    /// How often gateway should probe models with not the lowest response latency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_interval: Option<String>,
    /// The number of latency probes required to init moving average
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warmup_samples: Option<i32>,
}

/// Config for all available providers.
#[derive(Debug, Serialize, Deserialize)]
pub enum ProviderConfig {
    /// <https://anthropic.com/>
    #[serde(rename = "anthropic")]
    Anthropic(serde_json::Value),
    /// <https://azure.microsoft.com/en-us/products/ai-services/openai-service>
    #[serde(rename = "azureopenai")]
    AzureOpenAi(serde_json::Value),
    /// <https://aws.amazon.com/bedrock/>
    #[serde(rename = "bedrock")]
    Bedrock(serde_json::Value),
    /// <https://cohere.com/>
    #[serde(rename = "cohere")]
    Cohere(serde_json::Value),
    /// <https://octo.ai/>
    #[serde(rename = "octoml")]
    Octoml(serde_json::Value),
    /// <https://ollama.com/>
    #[serde(rename = "ollama")]
    Ollama(serde_json::Value),
    /// <https://openai.com/>
    #[serde(rename = "openai")]
    OpenAi(serde_json::Value),
}
