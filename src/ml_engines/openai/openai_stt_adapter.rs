use crate::data::config::SttConfig;
use crate::ml_engines::interfaces::stt_interface::SttInterface;
use std::error::Error;
use std::sync::Arc;

pub struct OpenAiSttAdapter {
    openai_client: Arc<openai_api_rust::OpenAI>,
    config: SttConfig,
}

impl SttInterface for OpenAiSttAdapter {
    fn recognize_speech(&self, audio: Vec<u8>, streaming: bool) -> Result<String, Box<dyn Error>> {
        todo!()
    }
}

impl OpenAiSttAdapter {
    pub fn new(openai_client: Arc<openai_api_rust::OpenAI>, config: SttConfig) -> Self {
        Self {
            openai_client,
            config,
        }
    }
}
