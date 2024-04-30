//! Language service, its request and response types.
//!

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::provider::Config as ProviderConfig;
use crate::Result;

/// `Glide` APIs for `/v1/language` endpoints.
#[derive(Clone)]
pub struct LanguageSvc(pub(crate) Arc<Config>);

impl LanguageSvc {
    /// Retrieves a list of all router configs.
    ///
    /// `GET /v1/language`
    pub async fn list(&self) -> Result<Vec<RouterConfig>> {
        #[derive(Debug, Deserialize)]
        struct RouterConfigs {
            pub routers: Vec<RouterConfig>,
        }

        let request = self.0.build(Method::GET, "/v1/language/");
        let response = self.0.send(request).await?;
        let content = response.json::<RouterConfigs>().await?;
        Ok(content.routers)
    }

    /// TODO.
    ///
    /// `POST /v1/language/{router}/chat`
    pub async fn chat(&self, router: &str, data: ChatRequest) -> Result<ChatResponse> {
        let path = format!("/v1/language/{router}/chat");

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request.json(&data)).await?;
        let content = response.json::<ChatResponse>().await?;

        Ok(content)
    }

    /// TODO.
    ///
    /// `GET /v1/language/{router}/chatStream`
    pub async fn stream(&self, router: &str) -> Result<()> {
        let path = format!("/v1/language/{router}/chatStream");

        // https://crates.io/crates/reqwest-websocket
        // https://crates.io/crates/tungstenite

        todo!()
    }
}

impl fmt::Debug for LanguageSvc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
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
    pub base_multiplier: i32,
    pub max_delay: i64,
    pub min_delay: i64,
    pub max_retries: i32,
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
    #[serde(flatten)]
    pub config: ProviderConfig,

    client: Option<ClientsConfig>,
    latency_config: Option<LatencyConfig>,
}

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct LatencyConfig {
    /// Weight of new latency measurements
    #[serde(rename = "decay", skip_serializing_if = "Option::is_none")]
    pub decay: Option<f64>,
    /// How often gateway should probe models with not the lowest response latency
    #[serde(rename = "update_interval", skip_serializing_if = "Option::is_none")]
    pub update_interval: Option<String>,
    /// The number of latency probes required to init moving average
    #[serde(rename = "warmup_samples", skip_serializing_if = "Option::is_none")]
    pub warmup_samples: Option<i32>,
}

/// TODO.
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    #[serde(rename = "message")]
    pub message: ChatMessage,
    #[serde(rename = "messageHistory")]
    pub message_history: Vec<ChatMessage>,
    #[serde(rename = "override")]
    pub r#override: Option<ChatMessageOverride>,
}

impl ChatRequest {
    /// Creates a new [`ChatRequest`].
    pub fn new(message: impl Into<ChatMessage>) -> Self {
        Self {
            message: message.into(),
            message_history: vec![],
            r#override: None,
        }
    }
}

/// TODO.
#[derive(Debug, Serialize)]
pub struct ChatMessageOverride {
    #[serde(rename = "message")]
    pub message: ChatMessage,
    #[serde(rename = "model_id")]
    pub model_id: String,
}

/// TODO.
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_response: Option<ModelResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router: Option<String>,
}

/// TODO.
#[derive(Debug, Deserialize)]
pub struct ModelResponse {
    #[serde(rename = "message")]
    pub message: Option<ChatMessage>,
    #[serde(rename = "responseId")]
    pub response_id: Option<HashMap<String, String>>,
    #[serde(rename = "tokenCount")]
    pub token_count: Option<TokenUsage>,
}

/// TODO.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    /// The content of the message.
    pub content: String,
    /// The name of the author of this message.
    ///
    /// May contain a-z, A-Z, 0-9, and underscores,
    /// with a maximum length of 64 characters.
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
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    #[default]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

/// TODO.
#[derive(Debug, Deserialize)]
pub struct TokenUsage {
    #[serde(rename = "promptTokens")]
    pub prompt_tokens: Option<i32>,
    #[serde(rename = "responseTokens")]
    pub response_tokens: Option<i32>,
    #[serde(rename = "totalTokens")]
    pub total_tokens: Option<i32>,
}

#[cfg(test)]
mod test {
    use crate::language::ChatRequest;
    use crate::{Client, Result};

    #[tokio::test]
    async fn list() -> Result<()> {
        let glide = Client::default();
        let response = glide.language.list().await?;
        assert_eq!(response.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn chat() -> Result<()> {
        let glide = Client::default();

        let router = "myrouter";
        let request = ChatRequest::new("Hello!");
        let _ = glide.language.chat(router, request).await?;

        Ok(())
    }

    #[tokio::test]
    async fn stream() -> Result<()> {
        let glide = Client::default();
        let _ = glide.language.stream("").await?;
        // TODO.
        Ok(())
    }
}
