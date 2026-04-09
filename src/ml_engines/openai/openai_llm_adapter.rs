use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use std::sync::Arc;

pub struct OpenAiLlmAdapter {
    openai_client: Arc<openai_api_rust::OpenAI>,
}

impl LlmInterface for OpenAiLlmAdapter {}

impl OpenAiLlmAdapter {
    pub fn new(openai_client: Arc<openai_api_rust::OpenAI>) -> Self {
        Self { openai_client }
    }
}
