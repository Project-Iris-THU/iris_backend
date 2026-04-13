use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait TtsInterface: Send + Sync {
    async fn generate_audio(
        &self,
        text: &str,
    ) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>>;
}
