use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use crate::ml_engines::interfaces::ocr_interface::OcrInterface;
use crate::ml_engines::interfaces::stt_interface::SttInterface;
use crate::ml_engines::interfaces::tts_interface::TtsInterface;
use std::iter::Map;

pub struct TlsConfig {
    enabled: bool,
    fullchain_path: String,
    key_path: String,
}

pub enum MLEngineType {
    OpenAI,
    Ollama,
}

pub struct MLEngineConfig {
    engine_type: MLEngineType,
    url: String,
    api_key: String,
}

pub struct SttConfig {
    model: String,
    engine_name: String,
    enabled: bool,
}

pub struct OcrConfig {
    model: String,
    engine_name: String,
    enabled: bool,
}

pub struct LlmConfig {
    model: String,
    engine_name: String,
    vision_model: bool,
    enabled: bool,
}

pub struct TtsConfig {
    model: String,
    engine_name: String,
    enabled: bool,
}

pub struct PipelineConfig {
    stt: SttConfig,
    ocr: OcrConfig,
    llm: LlmConfig,
    tts: TtsConfig,
}
pub struct ConfigData {
    host: String,
    port: u16,
    tls: TlsConfig,
    ml_engines: Map<String, MLEngineConfig>,
}

pub struct InterfaceConfig {
    stt_interface: Box<dyn SttInterface>,
    ocr_interface: Box<dyn OcrInterface>,
    llm_interface: Box<dyn LlmInterface>,
    tts_interface: Box<dyn TtsInterface>,
}
