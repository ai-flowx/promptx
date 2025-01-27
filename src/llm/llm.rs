use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Clone, Default)]
pub struct LLM {
    client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    key: String,
    endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    role: String,
    content: String,
}

impl LLM {
    pub fn new() -> Self {
        LLM {
            client: Client::new(),
        }
    }

    pub async fn chat_completion(&self, messages: Vec<Message>) -> Result<String, Box<dyn Error>> {
        let model_type = env::var("MODEL_TYPE").unwrap_or_else(|_| "openai".to_string());

        match model_type.as_str() {
            "openai" => self.call_openai_api(messages).await,
            _ => Err("Unsupported model type".into()),
        }
    }

    async fn call_openai_api(&self, messages: Vec<Message>) -> Result<String, Box<dyn Error>> {
        let api_key = env::var("OPENAI_API_KEY")?;
        let model = env::var("OPENAI_MODEL_NAME")?;

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&serde_json::json!({
                "model": model,
                "messages": messages,
                "temperature": 0.0
            }))
            .send()
            .await?;

        let response_data: serde_json::Value = response.json().await?;
        Ok(response_data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("Error processing response")
            .to_string())
    }

    pub fn list_model_type(&self) -> Vec<String> {
        // TBD: FIXME
        Vec::new()
    }
}

#[derive(Debug)]
pub enum ModelError {
    ApiError(String),
    ConfigError(String),
    AuthenticationError(String),
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelError::ApiError(msg) => write!(f, "API Error: {}", msg),
            ModelError::ConfigError(msg) => write!(f, "Configuration Error: {}", msg),
            ModelError::AuthenticationError(msg) => write!(f, "Authentication Error: {}", msg),
        }
    }
}

impl Error for ModelError {}
