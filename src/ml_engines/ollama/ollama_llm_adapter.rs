use crate::data::config::LlmConfig;
use crate::data::ml_engines::SystemPromptType;
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
        let system_prompt = match system_prompt_type {
            SystemPromptType::EasyLanguage => self.config.system_prompts.easy_language.clone(),
            SystemPromptType::VeryEasyLanguage => {
                self.config.system_prompts.very_easy_language.clone()
            }
            SystemPromptType::Summarize => self.config.system_prompts.summarize.clone(),
            SystemPromptType::CustomPrompt(prompt) => prompt,
        };

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
