use crate::data::config::LlmConfig;
use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use async_trait::async_trait;
use ollama_rs::generation::completion::request::GenerationRequest;
use std::error::Error;

pub struct OllamaLlmAdapter {
    ollama_client: ollama_rs::Ollama,
    config: LlmConfig,
}

#[async_trait]
impl LlmInterface for OllamaLlmAdapter {
    async fn generate_text(&self, prompt: String) -> Result<String, Box<dyn Error + Send + Sync>> {
        let request = GenerationRequest::new(self.config.model.clone(), prompt)
            .system(self.config.system_prompt.clone());

        let response = self.ollama_client.generate(request).await?;
        Ok(response.response)
    }
}

impl OllamaLlmAdapter {
    pub fn new(ollama_client: ollama_rs::Ollama, config: LlmConfig) -> Self {
        Self {
            ollama_client,
            config,
        }
    }
}
