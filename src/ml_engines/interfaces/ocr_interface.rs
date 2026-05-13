use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait OcrInterface: Send + Sync {
    async fn recognize_text<'a>(
        &self,
        image: Bytes,
        image_mime_type: &'a str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}
