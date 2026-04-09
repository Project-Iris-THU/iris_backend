use crate::ml_engines::interfaces::tts_interface::TtsInterface;
use std::sync::Arc;

pub struct OpenAiTtsAdapter {
    openai_client: Arc<openai_api_rust::OpenAI>,
}

impl TtsInterface for OpenAiTtsAdapter {}

impl OpenAiTtsAdapter {
    pub fn new(openai_client: Arc<openai_api_rust::OpenAI>) -> Self {
        Self { openai_client }
    }
}
