use crate::data::config::LlmConfig;
use crate::data::ml_engines::SystemPromptType;
use crate::ml_engines::helper_functions::system_prompts::match_system_prompt_type;
use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use async_trait::async_trait;
use futures_util::StreamExt;
use ollama_rs::generation::completion::request::GenerationRequest;
use std::error::Error;
use std::sync::mpsc;

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

    async fn generate_text_stream(
        &self,
        prompt: String,
        system_prompt_type: SystemPromptType,
        output_channel: mpsc::Sender<String>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let system_prompt =
            match_system_prompt_type(system_prompt_type, &self.config.system_prompts);

        let request =
            GenerationRequest::new(self.config.model.clone(), prompt).system(system_prompt);

        let mut stream = self.ollama_client.generate_stream(request).await?;

        let mut sentence_buffer = String::new();

        while let Some(res) = stream.next().await {
            let chunks = res?;
            for ele in chunks {
                let text = &ele.response;
                sentence_buffer.push_str(text);

                if text.contains(['.', '!', '?']) {
                    let sentence = sentence_buffer.trim().to_string();
                    if !sentence.is_empty() {
                        output_channel.send(sentence)?;
                        sentence_buffer.clear();
                    }
                }
            }
        }

        if !sentence_buffer.is_empty() {
            let _ = output_channel.send(sentence_buffer.trim().to_string())?;
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
}
