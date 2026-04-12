use crate::data::config::TtsConfig;
use crate::ml_engines::interfaces::tts_interface::TtsInterface;
use std::error::Error;
use std::sync::Arc;

pub struct OpenAiTtsAdapter {
    openai_client: Arc<openai_api_rust::OpenAI>,
    config: TtsConfig,
}

impl TtsInterface for OpenAiTtsAdapter {
    fn generate_audio(&self, text: &str, streaming: bool) -> Result<Vec<u8>, Box<dyn Error>> {
        todo!()
    }
}

impl OpenAiTtsAdapter {
    pub fn new(openai_client: Arc<openai_api_rust::OpenAI>, config: TtsConfig) -> Self {
        Self {
            openai_client,
            config,
        }
    }
}
