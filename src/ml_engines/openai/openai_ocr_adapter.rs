use crate::data::config::OcrConfig;
use crate::ml_engines::interfaces::ocr_interface::OcrInterface;
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestMessageContentPartImage, ChatCompletionRequestMessageContentPartText,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, ImageDetail,
        ImageUrl,
    },
};
use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use bytes::Bytes;
use std::error::Error;
use std::fmt::format;

pub struct OpenAiOcrAdapter {
    openai_client: Client<OpenAIConfig>,
    config: OcrConfig,
}

#[async_trait]
impl OcrInterface for OpenAiOcrAdapter {
    async fn recognize_text(
        &self,
        image: Bytes,
        image_mime_type: &String,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let image_base64 = format!("data:{};base64,{}", image_mime_type, STANDARD.encode(image));

        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(vec![
                ChatCompletionRequestMessageContentPartText::from(
                    "Extract the text from this image.",
                )
                .into(),
                ChatCompletionRequestMessageContentPartImage::from(ImageUrl {
                    url: image_base64,
                    detail: Some(ImageDetail::High),
                })
                .into(),
            ])
            .build()?;

        let request = CreateChatCompletionRequestArgs::default()
            .model(self.config.model.clone())
            .messages([user_message.into()])
            .build()?;

        let response = self.openai_client.chat().create(request).await?;

        let output_text = response
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .ok_or("No output text found in response")?;

        Ok(output_text)
    }
}

impl OpenAiOcrAdapter {
    pub fn new(openai_client: Client<OpenAIConfig>, config: OcrConfig) -> Self {
        Self {
            openai_client,
            config,
        }
    }
}
