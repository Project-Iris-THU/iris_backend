use crate::data::config::OcrConfig;
use crate::ml_engines::interfaces::ocr_interface::OcrInterface;
use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use bytes::Bytes;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::images::Image;
use std::error::Error;

pub struct OllamaOcrAdapter {
    ollama_client: Ollama,
    config: OcrConfig,
}

#[async_trait]
impl OcrInterface for OllamaOcrAdapter {
    async fn recognize_text(
        &self,
        image: Bytes,
        _: &String,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let image = Image::from_base64(STANDARD.encode(image));
        let request =
            GenerationRequest::new(self.config.model.clone(), self.config.system_prompt.clone())
                .add_image(image);

        let response = self.ollama_client.generate(request).await?;
        Ok(response.response)
    }
}

impl OllamaOcrAdapter {
    pub fn new(ollama_client: Ollama, config: OcrConfig) -> Self {
        Self {
            ollama_client,
            config,
        }
    }
}
