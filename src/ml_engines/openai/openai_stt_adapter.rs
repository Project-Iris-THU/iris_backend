use crate::data::config::SttConfig;
use crate::ml_engines::interfaces::stt_interface::SttInterface;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::audio::{AudioInput, CreateTranscriptionRequestArgs};
use async_trait::async_trait;
use bytes::Bytes;
use std::error::Error;

pub struct OpenAiSttAdapter {
    openai_client: Client<OpenAIConfig>,
    config: SttConfig,
}

#[async_trait]
impl SttInterface for OpenAiSttAdapter {
    async fn recognize_speech(&self, audio: Bytes) -> Result<String, Box<dyn Error + Send + Sync>> {
        let audio_file = AudioInput::from_bytes("audio".to_string(), audio);

        let request = CreateTranscriptionRequestArgs::default()
            .model(self.config.model.clone())
            .file(audio_file)
            .build()?;

        let response = self
            .openai_client
            .audio()
            .transcription()
            .create(request)
            .await?;

        Ok(response.text)
    }
}

impl OpenAiSttAdapter {
    pub fn new(openai_client: Client<OpenAIConfig>, config: SttConfig) -> Self {
        Self {
            openai_client,
            config,
        }
    }
}
