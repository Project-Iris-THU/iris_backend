use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use crate::ml_engines::interfaces::ocr_interface::OcrInterface;
use crate::ml_engines::interfaces::stt_interface::SttInterface;
use crate::ml_engines::interfaces::tts_interface::TtsInterface;
use std::collections::HashMap;

pub struct TlsConfig {
    pub enabled: bool,
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Clone)]
pub enum MLEngineType {
    OpenAI,
    Ollama,
}

pub struct MLEngineConfig {
    pub engine_type: MLEngineType,
    pub url: String,
    pub api_key: String,
}

pub struct SttConfig {
    pub model: String,
    pub engine_name: String,
    pub enabled: bool,
}

pub struct OcrConfig {
    pub model: String,
    pub engine_name: String,
    pub enabled: bool,
}

pub struct LlmConfig {
    pub model: String,
    pub engine_name: String,
    pub vision_model: bool,
    pub enabled: bool,
}

pub struct TtsConfig {
    pub model: String,
    pub engine_name: String,
    pub enabled: bool,
}

pub struct PipelineConfigs {
    pub stt: SttConfig,
    pub ocr: OcrConfig,
    pub llm: LlmConfig,
    pub tts: TtsConfig,
}
pub struct ConfigData {
    pub host: String,
    pub port: u16,
    pub tls: TlsConfig,
    pub ml_engines: HashMap<String, MLEngineConfig>,
    pub pipeline_configs: PipelineConfigs,
}

pub struct InterfaceConfig {
    pub stt_interface: Box<dyn SttInterface>,
    pub ocr_interface: Box<dyn OcrInterface>,
    pub llm_interface: Box<dyn LlmInterface>,
    pub tts_interface: Box<dyn TtsInterface>,
}
