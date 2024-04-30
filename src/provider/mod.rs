//! Service providers, configs and params.
//!

use serde::{Deserialize, Serialize};

pub mod anthropic;
pub mod azureopenai;
pub mod bedrock;
pub mod cohere;
pub mod octoml;
pub mod ollama;
pub mod openai;

/// Config for all available providers.
#[derive(Debug, Serialize, Deserialize)]
pub enum Config {
    #[serde(rename = "anthropic")]
    Anthropic(anthropic::AnthropicConfig),
    #[serde(rename = "azureopenai")]
    AzureOpenAi(azureopenai::AzureOpenAiConfig),
    #[serde(rename = "bedrock")]
    Bedrock(bedrock::BedrockConfig),
    #[serde(rename = "cohere")]
    Cohere(cohere::CohereConfig),
    #[serde(rename = "octoml")]
    Octoml(octoml::OctomlConfig),
    #[serde(rename = "ollama")]
    Ollama(ollama::OllamaConfig),
    #[serde(rename = "openai")]
    OpenAi(openai::OpenAiConfig),
}