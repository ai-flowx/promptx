use crate::config::config::{ConfigData, ConfigLLM};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Clone, Default)]
pub struct LLM {
    pub config: ConfigData,
    pub client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl LLM {
    pub fn new(config: ConfigData) -> Self {
        LLM {
            config,
            client: Client::new(),
        }
    }

    pub async fn chat_completion(
        &self,
        name: String,
        messages: Vec<Message>,
    ) -> Result<String, Box<dyn Error>> {
        if self.config.llm.is_empty() {
            return Err("No language model is configured".into());
        }

        let llm = self.config.llm.iter().find(|x| x.name == name).unwrap();

        match name.as_str() {
            "doubao" => self.call_openai_api(llm.clone(), messages).await,
            "openai" => self.call_openai_api(llm.clone(), messages).await,
            _ => Err("Unsupported llm name".into()),
        }
    }

    pub async fn call_openai_api(
        &self,
        config: ConfigLLM,
        messages: Vec<Message>,
    ) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .post(config.api.clone())
            .header("Authorization", format!("Bearer {}", config.key))
            .json(&serde_json::json!({
                "model": config.endpoint,
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
        if self.config.llm.is_empty() {
            return vec![];
        }

        self.config.llm.iter().map(|x| x.name.clone()).collect()
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
