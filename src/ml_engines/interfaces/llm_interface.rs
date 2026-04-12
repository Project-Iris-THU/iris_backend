use async_trait::async_trait;

#[async_trait]
pub trait LlmInterface {
    async fn generate_text(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
}
