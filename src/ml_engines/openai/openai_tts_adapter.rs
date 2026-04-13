use crate::data::config::TtsConfig;
use crate::ml_engines::interfaces::tts_interface::TtsInterface;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::audio::{CreateSpeechRequestArgs, SpeechModel};
use async_trait::async_trait;
use bytes::Bytes;
use std::error::Error;

pub struct OpenAiTtsAdapter {
    openai_client: Client<OpenAIConfig>,
    config: TtsConfig,
}

#[async_trait]
impl TtsInterface for OpenAiTtsAdapter {
    async fn generate_audio(&self, text: &str) -> Result<Bytes, Box<dyn Error + Send + Sync>> {
        let request = CreateSpeechRequestArgs::default()
            .input(text.to_string())
            .model(SpeechModel::Other(self.config.model.clone()))
            .build()?;

        let response = self.openai_client.audio().speech().create(request).await?;

        Ok(response.bytes)
    }
}

impl OpenAiTtsAdapter {
    pub fn new(openai_client: Client<OpenAIConfig>, config: TtsConfig) -> Self {
        Self {
            openai_client,
            config,
        }
    }
}
