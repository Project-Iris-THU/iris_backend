use crate::data::config::LlmConfig;
use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use async_trait::async_trait;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::{GenerationContext, GenerationResponse};
use std::error::Error;
use std::sync::Arc;

pub struct OllamaLlmAdapter {
    ollama_client: Arc<ollama_rs::Ollama>,
    config: LlmConfig,
}

#[async_trait]
impl LlmInterface for OllamaLlmAdapter {
    async fn generate_text(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let request = GenerationRequest::new(self.config.model.clone(), prompt)
            .system(self.config.system_prompt.clone());

        let response = self.ollama_client.generate(request).await?;
        Ok(response.response)
    }
}

impl OllamaLlmAdapter {
    pub fn new(ollama_client: Arc<ollama_rs::Ollama>, config: LlmConfig) -> Self {
        Self {
            ollama_client,
            config,
        }
    }
}
