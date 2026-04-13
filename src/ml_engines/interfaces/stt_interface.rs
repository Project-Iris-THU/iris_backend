use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait SttInterface: Send + Sync {
    async fn recognize_speech(
        &self,
        audio: Bytes,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}
