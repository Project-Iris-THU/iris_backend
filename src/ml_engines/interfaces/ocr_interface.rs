pub trait OcrInterface {
    fn recognize_text(
        &self,
        image: Vec<u8>,
        streaming: bool,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
