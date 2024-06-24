//! Request and response types for `/v1/language/{}/chat` endpoints.
//!

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Unified chat request across all language models.
#[must_use]
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    #[serde(rename = "message")]
    pub message: ChatMessage,
    #[serde(rename = "message_history", skip_serializing_if = "Option::is_none")]
    pub message_history: Option<Vec<ChatMessage>>,
    #[serde(rename = "override_params", skip_serializing_if = "Option::is_none")]
    pub override_params: Option<HashMap<String, ChatRequestOverride>>,
}

impl ChatRequest {
    /// Creates a new [`ChatRequest`].
    #[inline]
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

/// Unified chat response across all language models.
#[must_use]
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub cached: bool,
    pub created_at: i32,
    pub id: String,
    pub model_id: String,
    pub model_name: String,
    pub model_response: ModelResponse,
    pub provider_id: String,
    pub router_id: String,
}

impl ChatResponse {
    /// Returns the reference to the model response.
    #[inline]
    pub fn content(&self) -> &str {
        &self.model_response.message.content
    }
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
    pub role: Option<Role>,
}

impl ChatMessage {
    /// Creates a new [`ChatMessage`].
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_owned(),
            name: None,
            role: None,
        }
    }

    /// Overrides the default [`Role::User`] with [`Role::System`].
    pub const fn with_system(mut self) -> Self {
        self.role = Some(Role::System);
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
}
