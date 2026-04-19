use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use crate::ml_engines::interfaces::ocr_interface::OcrInterface;
use crate::ml_engines::interfaces::stt_interface::SttInterface;
use crate::ml_engines::interfaces::tts_interface::TtsInterface;
use std::collections::HashMap;
use std::sync::Arc;

pub struct TlsConfig {
    pub enabled: bool,
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MLEngineType {
    OpenAI,
    Ollama,
}

pub struct MLEngineConfig {
    pub engine_type: MLEngineType,
    pub url: String,
    pub api_key: String,
}

#[derive(Clone)]
pub struct SttConfig {
    pub model: String,
    pub engine_name: String,
    pub enabled: bool,
}

#[derive(Clone)]
pub struct OcrConfig {
    pub model: String,
    pub engine_name: String,
    pub system_prompt: String,
    pub enabled: bool,
}

#[derive(Clone)]
pub struct LlmSystemPrompts {
    pub easy_language: String,
    pub very_easy_language: String,
    pub summarize: String,
}

#[derive(Clone)]
pub struct LlmConfig {
    pub model: String,
    pub engine_name: String,
    pub vision_model: bool,
    pub system_prompts: LlmSystemPrompts,
    pub enabled: bool,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct InterfaceConfig {
    pub stt_interface: Arc<dyn SttInterface + Send + Sync>,
    pub ocr_interface: Arc<dyn OcrInterface + Send + Sync>,
    pub llm_interface: Arc<dyn LlmInterface + Send + Sync>,
    pub tts_interface: Arc<dyn TtsInterface + Send + Sync>,
}
