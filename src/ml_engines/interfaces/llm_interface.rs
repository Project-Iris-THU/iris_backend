use crate::data::ml_engines::SystemPromptType;
use async_trait::async_trait;

#[async_trait]
pub trait LlmInterface: Send + Sync {
    async fn generate_text(
        &self,
        prompt: String,
        system_prompt_type: SystemPromptType,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}
