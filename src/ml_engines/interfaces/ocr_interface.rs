use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait OcrInterface: Send + Sync {
    async fn recognize_text(
        &self,
        image: Bytes,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}
