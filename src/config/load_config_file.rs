use std::io::Read;
use crate::config::data::{
    ConfigData, LlmConfig, MLEngineConfig, MLEngineType, OcrConfig, SttConfig, TtsConfig, TlsConfig,
};
use log::info;
use rust_yaml::{Value, Yaml};

pub fn load_config_file<'config_data, R: Read>(
    reader: R,
    config_data: &'config_data mut ConfigData,
) -> Result<&'config_data mut ConfigData, Box<dyn std::error::Error>> {
    info!("Starting to load config file:");
    let yaml = Yaml::new();

    let parsed_data = match yaml.load(reader) {
        Ok(data) => data,
        Err(e) => Err(e)?,
    };

    match parsed_data.get_str("host") {
        Some(host) => config_data.host = match host.as_str() {
            Some(host) => host.to_string(),
            None => Err("Host is not a string")?,
        },
        None => Err("Host not found in config file")?,
    };

    match parsed_data.get_str("port") {
        Some(port) => config_data.port = port.to_string().parse::<u16>()?,
        None => Err("Port not found in config file")?,
    }

    match parsed_data.get_str("tls") {
        Some(tls_config) => {
            let enabled = match tls_config.get_str("enabled") {
                Some(enabled) => enabled.to_string().parse::<bool>()?,
                None => Err("TLS Enabled not found in config file")?,
            };

            let cert_path = match tls_config.get_str("cert_path") {
                Some(cert_path) => match cert_path.as_str() {
                    Some(cert_path) => cert_path.to_string(),
                    None => Err("TLS Cert Path is not a string")?,
                },
                None => Err("TLS Cert Path not found in config file")?,
            };

            let key_path = match tls_config.get_str("key_path") {
                Some(key_path) => match key_path.as_str() {
                    Some(key_path) => key_path.to_string(),
                    None => Err("TLS Key Path is not a string")?,
                },
                None => Err("TLS Key Path not found in config file")?,
            };

            config_data.tls = TlsConfig {
                enabled,
                cert_path,
                key_path,
            };
        }
        None => Err("TLS Config not found in config file")?,
    }

    match parsed_data.get_str("ml_engines") {
        Some(ml_engines) => {
            if let Some(ml_engines) = ml_engines.as_sequence() {
                for engine in ml_engines {
                    parse_ml_engine(engine, config_data)?;
                }
            } else {
                Err("ML Engines is not an array")?
            }
        }
        None => Err("ML Engines not found in config file")?,
    }

    let pipeline_configs = match parsed_data.get_str("pipeline_configs") {
        Some(pipeline_configs) => pipeline_configs,
        None => Err("Pipeline Configs not found in config file")?,
    };

    match pipeline_configs.get_str("stt") {
        Some(stt_config) => {
            let model = match stt_config.get_str("model") {
                Some(model) => match model.as_str() {
                    Some(model) => model.to_string(),
                    None => Err("STT Model is not a string")?,
                },
                None => Err("STT Model not found in config file")?,
            };

            let engine_name = match stt_config.get_str("engine_name") {
                Some(engine_name) => match engine_name.as_str() {
                    Some(engine_name) => engine_name.to_string(),
                    None => Err("STT Engine Name is not a string")?,
                },
                None => Err("STT Engine Name not found in config file")?,
            };

            let enabled = match stt_config.get_str("enabled") {
                Some(enabled) => enabled.to_string().parse::<bool>()?,
                None => Err("STT Enabled not found in config file")?,
            };

            config_data.pipeline_configs.stt = SttConfig {
                model,
                engine_name,
                enabled,
            };
        }
        None => Err("STT Config not found in config file")?,
    };

    match pipeline_configs.get_str("ocr") {
        Some(ocr_config) => {
            let model = match ocr_config.get_str("model") {
                Some(model) => match model.as_str() {
                    Some(model) => model.to_string(),
                    None => Err("OCR Model is not a string")?,
                },
                None => Err("OCR Model not found in config file")?,
            };

            let engine_name = match ocr_config.get_str("engine_name") {
                Some(engine_name) => match engine_name.as_str() {
                    Some(engine_name) => engine_name.to_string(),
                    None => Err("OCR Engine Name is not a string")?,
                },
                None => Err("OCR Engine Name not found in config file")?,
            };

            let enabled = match ocr_config.get_str("enabled") {
                Some(enabled) => enabled.to_string().parse::<bool>()?,
                None => Err("OCR Enabled not found in config file")?,
            };

            config_data.pipeline_configs.ocr = OcrConfig {
                model,
                engine_name,
                enabled,
            };
        }
        None => Err("OCR Config not found in config file")?,
    };

    match pipeline_configs.get_str("llm") {
        Some(llm_config) => {
            let model = match llm_config.get_str("model") {
                Some(model) => match model.as_str() {
                    Some(model) => model.to_string(),
                    None => Err("LLM Model is not a string")?,
                },
                None => Err("LLM Model not found in config file")?,
            };

            let engine_name = match llm_config.get_str("engine_name") {
                Some(engine_name) => match engine_name.as_str() {
                    Some(engine_name) => engine_name.to_string(),
                    None => Err("LLM Engine Name is not a string")?,
                },
                None => Err("LLM Engine Name not found in config file")?,
            };

            let vision_model = match llm_config.get_str("vision_model") {
                Some(vision_model) => vision_model.to_string().parse::<bool>()?,
                None => Err("LLM vision model not found in config file")?,
            };

            let enabled = match llm_config.get_str("enabled") {
                Some(enabled) => enabled.to_string().parse::<bool>()?,
                None => Err("LLM Enabled not found in config file")?,
            };

            config_data.pipeline_configs.llm = LlmConfig {
                model,
                vision_model,
                engine_name,
                enabled,
            };
        }
        None => Err("LLM Config not found in config file")?,
    };

    match pipeline_configs.get_str("tts") {
        Some(tts_config) => {
            let model = match tts_config.get_str("model") {
                Some(model) => match model.as_str() {
                    Some(model) => model.to_string(),
                    None => Err("TTS Model is not a string")?,
                },
                None => Err("TTS Model not found in config file")?,
            };

            let engine_name = match tts_config.get_str("engine_name") {
                Some(engine_name) => match engine_name.as_str() {
                    Some(engine_name) => engine_name.to_string(),
                    None => Err("TTS Engine Name is not a string")?,
                },
                None => Err("TTS Engine Name not found in config file")?,
            };

            let enabled = match tts_config.get_str("enabled") {
                Some(enabled) => enabled.to_string().parse::<bool>()?,
                None => Err("TTS Enabled not found in config file")?,
            };
            config_data.pipeline_configs.tts = TtsConfig {
                model,
                engine_name,
                enabled,
            };
        }
        None => Err("TTS Config not found in config file")?,
    };

    info!("Config file loaded successfully");
    Ok(config_data)
}

fn parse_ml_engine<'config_data>(
    engine: &Value,
    config_data: &'config_data mut ConfigData,
) -> Result<&'config_data mut ConfigData, Box<dyn std::error::Error>> {
    let name = match engine.get_str("name") {
        Some(name) => match name.as_str() {
            Some(name) => name.to_string(),
            None => Err("Name is not a string")?,
        },
        None => Err("Name not found in ML Engine config")?,
    };

    let engine_type = match engine.get_str("type") {
        Some(engine_type) => {
            if let Some(engine_type) = engine_type.as_str() {
                match engine_type {
                    "openai" => MLEngineType::OpenAI,
                    "ollama" => MLEngineType::Ollama,
                    _ => Err("Unsupported ML Engine type")?,
                }
            } else {
                Err("Type is not a string")?
            }
        }
        None => Err("Type not found in ML Engine config")?,
    };

    let url = match engine.get_str("url") {
        Some(url) => match url.as_str() {
            Some(url) => url.to_string(),
            None => Err("URL is not a string")?,
        },
        None => Err("URL not found in ML Engine config")?,
    };

    let api_key = match engine.get_str("api_key") {
        Some(api_key) => match api_key.as_str() {
            Some(api_key) => api_key.to_string(),
            None => Err("API Key is not a string")?,
        },
        None => Err("API Key not found in ML Engine config")?,
    };

    config_data.ml_engines.insert(
        name,
        MLEngineConfig {
            engine_type,
            url,
            api_key,
        },
    );

    Ok(config_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::data::{
        ConfigData, LlmConfig, MLEngineType, OcrConfig, PipelineConfigs, SttConfig, TlsConfig,
        TtsConfig,
    };

    #[test]
    fn test_load_correct_config_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut config_data = create_default_config_data();

        let test_config = r#"
host: "0.0.0.0"
port: 1234

tls:
  enabled: false
  cert_path: "some/path/to/fullchain.pem"
  key_path: "some/path/to/privkey.pem"

ml_engines:
  - name: my-openai
    type: openai
    url: https://api.openai.com/v1/
    api_key: "openai api key"
  - name: my-ollama
    type: ollama
    url: http://localhost:11434/
    api_key: "no api key"

pipeline_configs:
  stt:
    model: whisper-large-v3-turbo
    engine_name: my-openai
    enabled: true
  ocr:
    model: deepseek-ocr
    engine_name: my-ollama
    enabled: false
  llm:
    model: "qwen3-vl:8b"
    engine_name: my-ollama
    vision_model: true
    enabled: true
  tts:
    model: "qwen3-tts"
    engine_name: my-openai
    enabled: true
        "#;

        let reader = test_config.as_bytes();

        load_config_file(reader, &mut config_data)?;

        // Check general
        assert_eq!(config_data.host, "0.0.0.0");
        assert_eq!(config_data.port, 1234);

        // Check TLS
        assert_eq!(config_data.tls.enabled, false);
        assert_eq!(config_data.tls.cert_path, "some/path/to/fullchain.pem");
        assert_eq!(config_data.tls.key_path, "some/path/to/privkey.pem");

        // Check ML Engines
        assert_eq!(config_data.ml_engines.len(), 2);

        let my_openai = config_data.ml_engines.get("my-openai").unwrap();
        assert_eq!(my_openai.engine_type, MLEngineType::OpenAI);
        assert_eq!(my_openai.url, "https://api.openai.com/v1/");
        assert_eq!(my_openai.api_key, "openai api key");

        let my_ollama = config_data.ml_engines.get("my-ollama").unwrap();
        assert_eq!(my_ollama.engine_type, MLEngineType::Ollama);
        assert_eq!(my_ollama.url, "http://localhost:11434/");
        assert_eq!(my_ollama.api_key, "no api key");

        // Check pipeline configs
        assert_eq!(
            config_data.pipeline_configs.stt.model,
            "whisper-large-v3-turbo"
        );
        assert_eq!(config_data.pipeline_configs.stt.engine_name, "my-openai");
        assert_eq!(config_data.pipeline_configs.stt.enabled, true);

        assert_eq!(config_data.pipeline_configs.ocr.model, "deepseek-ocr");
        assert_eq!(config_data.pipeline_configs.ocr.engine_name, "my-ollama");
        assert_eq!(config_data.pipeline_configs.ocr.enabled, false);

        assert_eq!(config_data.pipeline_configs.llm.model, "qwen3-vl:8b");
        assert_eq!(config_data.pipeline_configs.llm.vision_model, true);
        assert_eq!(config_data.pipeline_configs.llm.engine_name, "my-ollama");
        assert_eq!(config_data.pipeline_configs.llm.enabled, true);

        assert_eq!(config_data.pipeline_configs.tts.model, "qwen3-tts");
        assert_eq!(config_data.pipeline_configs.tts.engine_name, "my-openai");
        assert_eq!(config_data.pipeline_configs.tts.enabled, true);

        Ok(())
    }

    fn create_default_config_data() -> ConfigData {
        ConfigData {
            host: "".to_string(),
            port: 0,
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
                    enabled: false,
                },
                llm: LlmConfig {
                    model: "".to_string(),
                    engine_name: "".to_string(),
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
}
