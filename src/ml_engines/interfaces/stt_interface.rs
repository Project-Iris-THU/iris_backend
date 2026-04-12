use async_trait::async_trait;

#[async_trait]
pub trait SttInterface {
    async fn recognize_speech(&self, audio: Vec<u8>) -> Result<String, Box<dyn std::error::Error>>;
}
