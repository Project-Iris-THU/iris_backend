use crate::data::config::LlmConfig;
use crate::data::ml_engines::SystemPromptType;
use crate::ml_engines::helper_functions::system_prompts::match_system_prompt_type;
use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use async_trait::async_trait;
use futures_util::StreamExt;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::models::ModelOptions;
use std::error::Error;
use tokio::sync::mpsc::Sender;

pub struct OllamaLlmAdapter {
    ollama_client: ollama_rs::Ollama,
    config: LlmConfig,
}

#[async_trait]
impl LlmInterface for OllamaLlmAdapter {
    async fn generate_text(
        &self,
        prompt: String,
        system_prompt_type: &SystemPromptType,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let system_prompt =
            match_system_prompt_type(system_prompt_type, &self.config.system_prompts);

        let options = self.set_options_from_config();

        let request = GenerationRequest::new(self.config.model.clone(), prompt)
            .system(system_prompt)
            .options(options);

        let response = self.ollama_client.generate(request).await?;
        Ok(response.response)
    }

    async fn generate_text_stream(
        &self,
        prompt: String,
        system_prompt_type: &SystemPromptType,
        output_channel: Sender<String>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let system_prompt =
            match_system_prompt_type(system_prompt_type, &self.config.system_prompts);

        let options = self.set_options_from_config();

        let request = GenerationRequest::new(self.config.model.clone(), prompt)
            .system(system_prompt)
            .options(options);

        let mut stream = self.ollama_client.generate_stream(request).await?;

        while let Some(res) = stream.next().await {
            let chunks = res?;
            for ele in chunks {
                let text = ele.response;

                output_channel.send(text).await?;
            }
        }

        Ok(())
    }
}

impl OllamaLlmAdapter {
    pub fn new(ollama_client: ollama_rs::Ollama, config: LlmConfig) -> Self {
        Self {
            ollama_client,
            config,
        }
    }

    fn set_options_from_config(&self) -> ModelOptions {
        let mut options = ModelOptions::default();

        if let Some(temp) = self.config.temperature {
            options = options.temperature(temp);
        };

        if let Some(top_p) = self.config.top_p {
            options = options.top_p(top_p);
        };

        if let Some(top_k) = self.config.top_k {
            options = options.top_k(top_k as u32);
        };

        options
    }
}
