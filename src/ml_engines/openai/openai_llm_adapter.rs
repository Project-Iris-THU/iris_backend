use crate::data::config::{LlmConfig, OcrConfig};
use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use std::error::Error;
use std::sync::Arc;

pub struct OpenAiLlmAdapter {
    openai_client: Arc<openai_api_rust::OpenAI>,
    config: LlmConfig,
}

impl LlmInterface for OpenAiLlmAdapter {
    fn generate_text(&self, prompt: &str, streaming: bool) -> Result<String, Box<dyn Error>> {
        todo!()
    }
}

impl OpenAiLlmAdapter {
    pub fn new(openai_client: Arc<openai_api_rust::OpenAI>, config: LlmConfig) -> Self {
        Self {
            openai_client,
            config,
        }
    }
}
