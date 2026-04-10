pub trait TtsInterface {
    fn generate_audio(
        &self,
        text: &str,
        streaming: bool,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}
