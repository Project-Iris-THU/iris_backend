use async_trait::async_trait;
use bytes::Bytes;
use tokio::sync::mpsc::Sender;

#[async_trait]
pub trait TtsInterface: Send + Sync {
    async fn generate_audio(
        &self,
        text: String,
    ) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>>;

    async fn generate_audio_stream(
        &self,
        text: String,
        output_channel: Sender<Bytes>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
