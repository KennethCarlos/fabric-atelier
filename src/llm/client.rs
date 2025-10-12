//! LLM client implementation.
//!
//! Calls external LLM APIs (OpenAI, Anthropic, etc.) to process patterns.

use crate::error::{Error, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, info};

/// LLM client for processing patterns.
pub struct LlmClient {
    /// HTTP client.
    client: Client,
    
    /// API provider (openai, anthropic).
    provider: String,
    
    /// API key.
    api_key: String,
    
    /// Model name.
    model: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Debug, Deserialize)]
struct MessageContent {
    content: String,
}

impl LlmClient {
    /// Create a new LLM client.
    ///
    /// # Arguments
    ///
    /// * `provider` - API provider (openai, anthropic)
    /// * `api_key` - API key for the provider
    /// * `model` - Model name to use
    pub fn new(provider: String, api_key: String, model: String) -> Self {
        Self {
            client: Client::new(),
            provider,
            api_key,
            model,
        }
    }

    /// Process a pattern with the given content.
    ///
    /// # Arguments
    ///
    /// * `system_prompt` - The pattern's system prompt
    /// * `user_content` - The user's content to process
    ///
    /// # Returns
    ///
    /// The generated output from the LLM.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    pub async fn process(
        &self,
        system_prompt: &str,
        user_content: &str,
    ) -> Result<String> {
        match self.provider.as_str() {
            "openai" => self.process_openai(system_prompt, user_content).await,
            "anthropic" => self.process_anthropic(system_prompt, user_content).await,
            "ollama" => self.process_ollama(system_prompt, user_content).await,
            _ => Err(Error::ConfigError(format!(
                "Unsupported LLM provider: {}",
                self.provider
            ))),
        }
    }

    /// Process using OpenAI API.
    async fn process_openai(
        &self,
        system_prompt: &str,
        user_content: &str,
    ) -> Result<String> {
        debug!("Calling OpenAI API with model: {}", self.model);

        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: user_content.to_string(),
                },
            ],
            temperature: 0.7,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::ConfigError(format!("OpenAI API request failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigError(format!(
                "OpenAI API error {}: {}",
                status, error_text
            )));
        }

        let response_data: OpenAIResponse = response
            .json()
            .await
            .map_err(|e| Error::ConfigError(format!("Failed to parse OpenAI response: {e}")))?;

        let content = response_data
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| Error::ConfigError("No response from OpenAI".to_string()))?;

        info!("OpenAI API call successful, response length: {} bytes", content.len());
        Ok(content)
    }

    /// Process using Anthropic API.
    async fn process_anthropic(
        &self,
        system_prompt: &str,
        user_content: &str,
    ) -> Result<String> {
        debug!("Calling Anthropic API with model: {}", self.model);

        let request = json!({
            "model": self.model,
            "max_tokens": 4096,
            "system": system_prompt,
            "messages": [{
                "role": "user",
                "content": user_content
            }]
        });

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::ConfigError(format!("Anthropic API request failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigError(format!(
                "Anthropic API error {}: {}",
                status, error_text
            )));
        }

        let response_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::ConfigError(format!("Failed to parse Anthropic response: {e}")))?;

        let content = response_data["content"][0]["text"]
            .as_str()
            .ok_or_else(|| Error::ConfigError("No response from Anthropic".to_string()))?
            .to_string();

        info!("Anthropic API call successful, response length: {} bytes", content.len());
        Ok(content)
    }

    /// Process using Ollama API (local).
    async fn process_ollama(
        &self,
        system_prompt: &str,
        user_content: &str,
    ) -> Result<String> {
        debug!("Calling Ollama API with model: {}", self.model);

        let request = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": user_content
                }
            ],
            "stream": false
        });

        // Ollama default endpoint
        let ollama_url = std::env::var("OLLAMA_HOST")
            .unwrap_or_else(|_| "http://localhost:11434".to_string());
        let url = format!("{}/api/chat", ollama_url);

        debug!("Ollama endpoint: {}", url);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::ConfigError(format!("Ollama API request failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigError(format!(
                "Ollama API error {}: {}",
                status, error_text
            )));
        }

        let response_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::ConfigError(format!("Failed to parse Ollama response: {e}")))?;

        let content = response_data["message"]["content"]
            .as_str()
            .ok_or_else(|| Error::ConfigError("No response from Ollama".to_string()))?
            .to_string();

        info!("Ollama API call successful, response length: {} bytes", content.len());
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = LlmClient::new(
            "openai".to_string(),
            "test-key".to_string(),
            "gpt-4".to_string(),
        );
        assert_eq!(client.provider, "openai");
        assert_eq!(client.model, "gpt-4");
    }

    #[test]
    fn test_ollama_client() {
        let client = LlmClient::new(
            "ollama".to_string(),
            "".to_string(), // No API key needed for Ollama
            "llama2".to_string(),
        );
        assert_eq!(client.provider, "ollama");
    }
}
