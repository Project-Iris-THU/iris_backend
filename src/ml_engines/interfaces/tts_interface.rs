use async_trait::async_trait;

#[async_trait]
pub trait TtsInterface {
    async fn generate_audio(&self, text: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}
