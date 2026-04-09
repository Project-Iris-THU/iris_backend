use crate::ml_engines::interfaces::stt_interface::SttInterface;
use std::sync::Arc;

pub struct OpenAiSttAdapter {
    openai_client: Arc<openai_api_rust::OpenAI>,
}

impl SttInterface for OpenAiSttAdapter {}

impl OpenAiSttAdapter {
    pub fn new(openai_client: Arc<openai_api_rust::OpenAI>) -> Self {
        Self { openai_client }
    }
}
