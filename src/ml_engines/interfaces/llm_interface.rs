use crate::data::ml_engines::SystemPromptType;
use async_trait::async_trait;
use std::error::Error;
use tokio::sync::mpsc::Sender;

#[async_trait]
pub trait LlmInterface: Send + Sync {
    async fn generate_text(
        &self,
        prompt: String,
        system_prompt_type: &SystemPromptType,
    ) -> Result<String, Box<dyn Error + Send + Sync>>;

    async fn generate_text_stream(
        &self,
        prompt: String,
        system_prompt_type: &SystemPromptType,
        output_channel: Sender<String>,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}
