use crate::data::config::LlmConfig;
use crate::data::ml_engines::SystemPromptType;
use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use async_openai::{Client, config::OpenAIConfig, types::responses::CreateResponseArgs};
use async_trait::async_trait;
use std::error::Error;

pub struct OpenAiLlmAdapter {
    openai_client: Client<OpenAIConfig>,
    config: LlmConfig,
}

#[async_trait]
impl LlmInterface for OpenAiLlmAdapter {
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

        let request = CreateResponseArgs::default()
            .model(self.config.model.clone())
            .prompt(system_prompt)
            .input(prompt)
            .build()?;

        let response = self.openai_client.responses().create(request).await?;

        let output_text = match response.output_text() {
            Some(text) => text,
            None => return Err("No output text found in response".into()),
        };

        Ok(output_text)
    }
}

impl OpenAiLlmAdapter {
    pub fn new(openai_client: Client<OpenAIConfig>, config: LlmConfig) -> Self {
        Self {
            openai_client,
            config,
        }
    }
}
