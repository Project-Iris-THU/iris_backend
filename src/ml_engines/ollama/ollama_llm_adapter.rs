use crate::data::config::LlmConfig;
use crate::data::ml_engines::SystemPromptType;
use crate::ml_engines::helper_functions::system_prompts::match_system_prompt_type;
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
    async fn generate_text(
        &self,
        prompt: String,
        system_prompt_type: SystemPromptType,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let system_prompt =
            match_system_prompt_type(system_prompt_type, &self.config.system_prompts);

        let request =
            GenerationRequest::new(self.config.model.clone(), prompt).system(system_prompt);

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
