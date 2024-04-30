use serde::{Deserialize, Serialize};

pub mod openai;
pub mod anthropic;
pub mod ollama;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Config {
    Anthropic(anthropic::AnthropicConfig),
    // pub azureopenai: Option<Box<models::AzureopenaiPeriodConfig>>,
    // pub bedrock: Option<Box<models::BedrockPeriodConfig>>,
    // pub client: Option<Box<models::ClientsPeriodClientConfig>>,
    // pub cohere: Option<Box<models::CoherePeriodConfig>>,
    // pub latency: Option<Box<models::LatencyPeriodConfig>>,
    // pub octoml: Option<Box<models::OctomlPeriodConfig>>,
    Ollama(ollama::OllamaConfig),
    OpenAi(openai::OpenAiConfig),
}
