//! Language service, its request and response types.
//!

use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use types::{ChatRequest, ChatResponse};

use crate::config::Config;
use crate::Result;

/// `Glide` APIs for `/v1/language` endpoints.
#[derive(Clone)]
pub struct Language(pub(crate) Arc<Config>);

impl Language {
    /// Retrieves a list of all router configs.
    ///
    /// `GET /v1/language`
    pub async fn list(&self) -> Result<Vec<types::RouterConfig>> {
        let request = self.0.build(Method::GET, "/v1/language/");
        let response = self.0.send(request).await?;
        let content = response.json::<types::RouterConfigs>().await?;
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

impl fmt::Debug for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub mod types {
    use std::collections::HashMap;

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

        // pub anthropic: Option<AnthropicConfig>,
        // pub azureopenai: Option<Box<models::AzureopenaiPeriodConfig>>,
        // pub bedrock: Option<Box<models::BedrockPeriodConfig>>,
        // pub client: Option<Box<models::ClientsPeriodClientConfig>>,
        // pub cohere: Option<Box<models::CoherePeriodConfig>>,
        // pub latency: Option<Box<models::LatencyPeriodConfig>>,
        // pub octoml: Option<Box<models::OctomlPeriodConfig>>,
        // pub ollama: Option<OllamaConfig>,
        // pub openai: Option<OpenAiConfig>,
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
        pub fn new() -> Self {
            todo!()
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
        #[serde(rename = "cached", skip_serializing_if = "Option::is_none")]
        pub cached: Option<bool>,
        #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
        pub created: Option<i32>,
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
        pub model: Option<String>,
        #[serde(rename = "model_id", skip_serializing_if = "Option::is_none")]
        pub model_id: Option<String>,
        #[serde(rename = "modelResponse", skip_serializing_if = "Option::is_none")]
        pub model_response: Option<ModelResponse>,
        #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(rename = "router", skip_serializing_if = "Option::is_none")]
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
        pub role: String,
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

        let router = "";
        let request = todo!();
        let response = glide.language.chat(router, request).await?;

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
