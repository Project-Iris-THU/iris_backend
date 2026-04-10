pub trait SttInterface {
    fn recognize_speech(
        &self,
        audio: Vec<u8>,
        streaming: bool,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
