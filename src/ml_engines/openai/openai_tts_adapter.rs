use crate::data::config::TtsConfig;
use crate::ml_engines::interfaces::tts_interface::TtsInterface;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::traits::EventType;
use async_openai::types::audio::{
    CreateSpeechRequest, CreateSpeechRequestArgs, CreateSpeechResponseStreamEvent, SpeechModel,
    Voice,
};
use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use bytes::Bytes;
use futures_util::StreamExt;
use log::{debug, error};
use std::error::Error;
use tokio::sync::mpsc::Sender;

pub struct OpenAiTtsAdapter {
    openai_client: Client<OpenAIConfig>,
    config: TtsConfig,
}

#[async_trait]
impl TtsInterface for OpenAiTtsAdapter {
    async fn generate_audio(&self, text: String) -> Result<Bytes, Box<dyn Error + Send + Sync>> {
        let request = Self::create_custom_speech_request(
            text,
            self.config.model.clone(),
            self.config.voice.clone(),
        )?;

        let response = self.openai_client.audio().speech().create(request).await?;

        Ok(response.bytes)
    }

    async fn generate_audio_stream(
        &self,
        text: String,
        output_channel: Sender<Bytes>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let request = Self::create_custom_speech_request(
            text,
            self.config.model.clone(),
            self.config.voice.clone(),
        )?;

        let mut stream = self
            .openai_client
            .audio()
            .speech()
            .create_stream(request)
            .await?;

        while let Some(result) = stream.next().await {
            match result {
                Ok(response_event) => match response_event {
                    CreateSpeechResponseStreamEvent::SpeechAudioDelta(delta) => {
                        let audio_base64 = delta.audio;

                        let audio = STANDARD.decode(audio_base64)?;

                        output_channel.send(Bytes::from(audio)).await?;
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

        Ok(())
    }
}

impl OpenAiTtsAdapter {
    pub fn new(openai_client: Client<OpenAIConfig>, config: TtsConfig) -> Self {
        Self {
            openai_client,
            config,
        }
    }

    fn create_custom_speech_request(
        text: String,
        model: String,
        voice: String,
    ) -> Result<CreateSpeechRequest, Box<dyn Error + Send + Sync>> {
        Ok(CreateSpeechRequestArgs::default()
            .input(text)
            .model(SpeechModel::Other(model))
            .voice(Voice::Other(voice))
            .build()?)
    }
}
