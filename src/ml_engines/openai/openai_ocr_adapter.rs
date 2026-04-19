use crate::data::config::OcrConfig;
use crate::ml_engines::interfaces::ocr_interface::OcrInterface;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::responses::CreateResponseArgs;
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

        let request = CreateResponseArgs::default()
            .model(self.config.model.clone())
            .prompt(self.config.system_prompt.clone())
            .input(image_base64)
            .build()?;

        let response = self.openai_client.responses().create(request).await?;

        let output_text = match response.output_text() {
            Some(text) => text,
            None => return Err("No output text found in response".into()),
        };

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
