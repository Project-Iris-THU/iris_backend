use crate::data::config::{
    ConfigData, LlmConfig, OcrConfig, PipelineConfigs, SttConfig, TlsConfig, TtsConfig,
};

pub const OLLAMA_DEFAULT_PORT: u16 = 11434;

pub fn create_default_config_data() -> ConfigData {
    ConfigData {
        host: "0.0.0.0".to_string(),
        port: 8080,
        tls: TlsConfig {
            enabled: false,
            cert_path: "".to_string(),
            key_path: "".to_string(),
        },
        ml_engines: Default::default(),
        pipeline_configs: PipelineConfigs {
            stt: SttConfig {
                model: "".to_string(),
                engine_name: "".to_string(),
                enabled: false,
            },
            ocr: OcrConfig {
                model: "".to_string(),
                engine_name: "".to_string(),
                system_prompt: "".to_string(),
                enabled: false,
            },
            llm: LlmConfig {
                model: "".to_string(),
                engine_name: "".to_string(),
                system_prompt: "".to_string(),
                vision_model: false,
                enabled: false,
            },
            tts: TtsConfig {
                model: "".to_string(),
                engine_name: "".to_string(),
                enabled: false,
            },
        },
    }
}
