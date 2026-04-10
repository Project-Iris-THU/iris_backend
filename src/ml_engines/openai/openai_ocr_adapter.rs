use crate::ml_engines::interfaces::ocr_interface::OcrInterface;
use std::error::Error;
use std::sync::Arc;

pub struct OpenAiOcrAdapter {
    openai_client: Arc<openai_api_rust::OpenAI>,
}

impl OcrInterface for OpenAiOcrAdapter {
    fn recognize_text(&self, image: Vec<u8>, streaming: bool) -> Result<String, Box<dyn Error>> {
        todo!()
    }
}

impl OpenAiOcrAdapter {
    pub fn new(openai_client: Arc<openai_api_rust::OpenAI>) -> Self {
        Self { openai_client }
    }
}
