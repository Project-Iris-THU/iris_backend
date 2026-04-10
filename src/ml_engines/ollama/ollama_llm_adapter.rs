use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use std::error::Error;
use std::sync::Arc;
pub struct OllamaLlmAdapter {
    ollama_client: Arc<ollama_rs::Ollama>,
}

impl LlmInterface for OllamaLlmAdapter {
    fn generate_text(&self, prompt: &str, streaming: bool) -> Result<String, Box<dyn Error>> {
        todo!()
    }
}

impl OllamaLlmAdapter {
    pub fn new(ollama_client: Arc<ollama_rs::Ollama>) -> Self {
        Self { ollama_client }
    }
}
