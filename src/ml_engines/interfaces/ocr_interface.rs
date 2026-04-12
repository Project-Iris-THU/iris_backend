use async_trait::async_trait;

#[async_trait]
pub trait OcrInterface {
    async fn recognize_text(&self, image: Vec<u8>) -> Result<String, Box<dyn std::error::Error>>;
}
