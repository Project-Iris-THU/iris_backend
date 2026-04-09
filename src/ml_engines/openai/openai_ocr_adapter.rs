use crate::ml_engines::interfaces::ocr_interface::OcrInterface;
use std::sync::Arc;

pub struct OpenAiOcrAdapter {
    openai_client: Arc<openai_api_rust::OpenAI>,
}

impl OcrInterface for OpenAiOcrAdapter {}

impl OpenAiOcrAdapter {
    pub fn new(openai_client: Arc<openai_api_rust::OpenAI>) -> Self {
        Self { openai_client }
    }
}
