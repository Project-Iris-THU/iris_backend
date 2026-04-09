use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use std::sync::Arc;
pub struct OllamaLlmAdapter {
    ollama_client: Arc<ollama_rs::Ollama>,
}

impl LlmInterface for OllamaLlmAdapter {}

impl OllamaLlmAdapter {
    pub fn new(ollama_client: Arc<ollama_rs::Ollama>) -> Self {
        Self { ollama_client }
    }
}
