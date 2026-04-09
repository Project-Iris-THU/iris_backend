use crate::ml_engines::interfaces::ocr_interface::OcrInterface;
use std::sync::Arc;

pub struct OllamaOcrAdapter {
    ollama_client: Arc<ollama_rs::Ollama>,
}

impl OcrInterface for OllamaOcrAdapter {}

impl OllamaOcrAdapter {
    pub fn new(ollama_client: Arc<ollama_rs::Ollama>) -> Self {
        Self { ollama_client }
    }
}
