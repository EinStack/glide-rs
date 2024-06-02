//! Request and response types for `/v1/language` endpoints.
//!

use std::collections::HashMap;

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

/// Unified chat request across all language models.
#[must_use]
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    #[serde(rename = "message")]
    pub message: ChatMessage,
    #[serde(rename = "message_history", skip_serializing_if = "Option::is_none")]
    pub message_history: Option<Vec<ChatMessage>>,
    #[serde(rename = "override_params", skip_serializing_if = "Option::is_none")]
    pub override_params: Option<ChatRequestOverride>,
}

impl ChatRequest {
    /// Creates a new [`ChatRequest`].
    pub fn new(message: impl Into<ChatMessage>) -> Self {
        Self::from(message.into())
    }
}

impl<T> From<T> for ChatRequest
where
    T: Into<ChatMessage>,
{
    fn from(message: T) -> Self {
        Self {
            message: message.into(),
            message_history: None,
            override_params: None,
        }
    }
}

/// Content and role of the message.
#[must_use]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    /// The content of the message.
    pub content: String,
    /// The name of the author of this message.
    ///
    /// May contain a-z, A-Z, 0-9, and underscores,
    /// with a maximum length of 64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The role of the author of this message.
    ///
    /// One of system, user, or assistant.
    pub role: Role,
}

impl ChatMessage {
    /// Creates a new [`ChatMessage`].
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_owned(),
            name: None,
            role: Role::User,
        }
    }

    /// Overrides the default [`Role::User`] with [`Role::System`].
    pub fn with_system(mut self) -> Self {
        self.role = Role::System;
        self
    }
}

impl From<String> for ChatMessage {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

impl From<&str> for ChatMessage {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

/// The role of the author of this message.
///
/// One of system, user, or assistant.
#[must_use]
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[default]
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

/// Override of a single chat request.
#[derive(Debug, Serialize)]
pub struct ChatRequestOverride {
    #[serde(rename = "message")]
    pub message: ChatMessage,
    #[serde(rename = "model_id")]
    pub model_id: String,
}

/// Unified chat response across all language models.
#[must_use]
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub cached: Option<bool>,
    pub created_at: Option<i32>,
    pub id: Option<String>,
    pub model_id: Option<String>,
    pub model_name: Option<String>,
    pub model_response: Option<ModelResponse>,
    pub provider_id: Option<String>,
    pub router_id: Option<String>,
}

/// Unified response from the provider.
#[derive(Debug, Deserialize)]
pub struct ModelResponse {
    pub message: ChatMessage,
    pub metadata: Option<HashMap<String, String>>,
    pub token_count: TokenUsage,
}

/// Prompt, response and total token usage.
#[derive(Debug, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: i32,
    pub response_tokens: i32,
    pub total_tokens: i32,
}
