use crate::data::config::{MLEngineConfig, OcrConfig};
use crate::ml_engines::interfaces::ocr_interface::OcrInterface;
use std::error::Error;
use std::sync::Arc;

pub struct OllamaOcrAdapter {
    ollama_client: Arc<ollama_rs::Ollama>,
    config: OcrConfig,
}

impl OcrInterface for OllamaOcrAdapter {
    fn recognize_text(&self, image: Vec<u8>, streaming: bool) -> Result<String, Box<dyn Error>> {
        todo!()
    }
}

impl OllamaOcrAdapter {
    pub fn new(ollama_client: Arc<ollama_rs::Ollama>, config: OcrConfig) -> Self {
        Self {
            ollama_client,
            config,
        }
    }
}
