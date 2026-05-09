use crate::data::config::LlmConfig;
use crate::data::ml_engines::SystemPromptType;
use crate::ml_engines::helper_functions::system_prompts::match_system_prompt_type;
use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use async_openai::traits::EventType;
use async_openai::types::responses::ResponseStreamEvent;
use async_openai::{Client, config::OpenAIConfig, types::responses::CreateResponseArgs};
use async_trait::async_trait;
use futures_util::StreamExt;
use log::{debug, error};
use std::error::Error;
use tokio::sync::mpsc::Sender;

pub struct OpenAiLlmAdapter {
    openai_client: Client<OpenAIConfig>,
    config: LlmConfig,
}

#[async_trait]
impl LlmInterface for OpenAiLlmAdapter {
    async fn generate_text(
        &self,
        prompt: String,
        system_prompt_type: &SystemPromptType,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let system_prompt =
            match_system_prompt_type(system_prompt_type, &self.config.system_prompts);

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

    async fn generate_text_stream(
        &self,
        prompt: String,
        system_prompt_type: &SystemPromptType,
        output_channel: Sender<String>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let system_prompt =
            match_system_prompt_type(system_prompt_type, &self.config.system_prompts);

        let request = CreateResponseArgs::default()
            .model(self.config.model.clone())
            .prompt(system_prompt)
            .input(prompt)
            .build()?;

        let mut stream = self
            .openai_client
            .responses()
            .create_stream(request)
            .await?;

        let mut sentence_buffer = String::new();

        while let Some(result) = stream.next().await {
            match result {
                Ok(response_event) => match &response_event {
                    ResponseStreamEvent::ResponseOutputTextDelta(delta) => {
                        let text = &delta.delta;

                        sentence_buffer.push_str(text);

                        if text.contains(['.', '!', '?']) {
                            let sentence = sentence_buffer.trim().to_string();
                            if !sentence.is_empty() {
                                output_channel.send(sentence).await?;
                                sentence_buffer.clear();
                            }
                        }
                    }
                    _ => {
                        debug!("\n{}: skipping\n", response_event.event_type());
                    }
                },
                Err(e) => {
                    error!("\n{e:#?}");
                }
            }
        }

        if !sentence_buffer.is_empty() {
            let _ = output_channel
                .send(sentence_buffer.trim().to_string())
                .await?;
        }

        Ok(())
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
