use crate::data::config::LlmConfig;
use crate::data::ml_engines::SystemPromptType;
use crate::ml_engines::helper_functions::system_prompts::match_system_prompt_type;
use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use async_openai::types::chat::{
    ChatCompletionRequestMessageContentPartText, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_openai::{Client, config::OpenAIConfig};
use async_trait::async_trait;
use futures_util::StreamExt;
use log::error;
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

        let system_message = ChatCompletionRequestSystemMessageArgs::default()
            .content(ChatCompletionRequestSystemMessageContent::from(
                system_prompt,
            ))
            .build()?;

        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(vec![
                ChatCompletionRequestMessageContentPartText::from(prompt).into(),
            ])
            .build()?;

        let mut request_args = CreateChatCompletionRequestArgs::default();

        self.set_options_from_config(&mut request_args);

        let request = request_args
            .model(self.config.model.clone())
            .messages([system_message.into(), user_message.into()])
            .build()?;

        let response = self.openai_client.chat().create(request).await?;

        let output_text = response
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .ok_or("No output text found in response")?;

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

        let system_message = ChatCompletionRequestSystemMessageArgs::default()
            .content(ChatCompletionRequestSystemMessageContent::from(
                system_prompt,
            ))
            .build()?;

        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(vec![
                ChatCompletionRequestMessageContentPartText::from(prompt).into(),
            ])
            .build()?;

        let mut request_args = CreateChatCompletionRequestArgs::default();

        self.set_options_from_config(&mut request_args);

        let request = request_args
            .model(self.config.model.clone())
            .messages([system_message.into(), user_message.into()])
            .build()?;

        let mut stream = self.openai_client.chat().create_stream(request).await?;

        while let Some(result) = stream.next().await {
            match result {
                Ok(response_event) => {
                    let text = response_event
                        .choices
                        .first()
                        .and_then(|choice| choice.delta.content.clone())
                        .ok_or("No output text found in response")?;

                    output_channel.send(text).await?;
                }
                Err(e) => {
                    error!("\n{e:#?}");
                }
            }
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

    fn set_options_from_config(&self, args: &mut CreateChatCompletionRequestArgs) {
        if let Some(temp) = self.config.temperature {
            args.temperature(temp);
        };

        if let Some(top_p) = self.config.top_p {
            args.top_p(top_p);
        };
    }
}
