pub trait LlmInterface {
    fn generate_text(
        &self,
        prompt: &str,
        streaming: bool,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
