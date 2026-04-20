use crate::data::config::{
    ConfigData, LlmConfig, LlmSystemPrompts, MLEngineConfig, MLEngineType, OcrConfig, SttConfig,
    TlsConfig, TtsConfig,
};
use log::info;
use rust_yaml::{Value, Yaml};
use std::io::Read;

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

    config_data.host = get_string_from_value(&parsed_data, "host")?;

    config_data.port = get_u16_from_value(&parsed_data, "port")?;

    match parsed_data.get_str("tls") {
        Some(tls_config) => {
            let enabled = get_bool_from_value(tls_config, "enabled")?;

            let cert_path = get_string_from_value(tls_config, "cert_path")?;

            let key_path = get_string_from_value(tls_config, "key_path")?;

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
            let model = get_string_from_value(stt_config, "model")?;

            let engine_name = get_string_from_value(stt_config, "engine_name")?;

            let enabled = get_bool_from_value(stt_config, "enabled")?;

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
            let model = get_string_from_value(ocr_config, "model")?;

            let engine_name = get_string_from_value(ocr_config, "engine_name")?;

            let system_prompt = get_string_from_value(ocr_config, "system_prompt")?;

            let enabled = get_bool_from_value(ocr_config, "enabled")?;

            config_data.pipeline_configs.ocr = OcrConfig {
                model,
                engine_name,
                system_prompt,
                enabled,
            };
        }
        None => Err("OCR Config not found in config file")?,
    };

    match pipeline_configs.get_str("llm") {
        Some(llm_config) => {
            let model = get_string_from_value(llm_config, "model")?;

            let engine_name = get_string_from_value(llm_config, "engine_name")?;

            let system_prompts = match llm_config.get_str("system_prompts") {
                Some(system_prompts) => {
                    let easy_language = get_string_from_value(system_prompts, "easy_language")?;
                    let very_easy_language =
                        get_string_from_value(system_prompts, "very_easy_language")?;
                    let summarize = get_string_from_value(system_prompts, "summarize")?;

                    LlmSystemPrompts {
                        easy_language,
                        very_easy_language,
                        summarize,
                    }
                }
                None => Err("System prompts not found in config file")?,
            };

            let vision_model = get_bool_from_value(llm_config, "vision_model")?;

            let enabled = get_bool_from_value(llm_config, "enabled")?;

            config_data.pipeline_configs.llm = LlmConfig {
                model,
                vision_model,
                engine_name,
                system_prompts,
                enabled,
            };
        }
        None => Err("LLM Config not found in config file")?,
    };

    match pipeline_configs.get_str("tts") {
        Some(tts_config) => {
            let model = get_string_from_value(tts_config, "model")?;

            let engine_name = get_string_from_value(tts_config, "engine_name")?;

            let enabled = get_bool_from_value(tts_config, "enabled")?;

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
    let name = get_string_from_value(engine, "name")?;

    let engine_type = match get_string_from_value(engine, "type")?.as_str() {
        "openai" => MLEngineType::OpenAI,
        "ollama" => MLEngineType::Ollama,
        _ => Err("Unsupported ML Engine type")?,
    };

    let url = get_string_from_value(engine, "url")?;

    let api_key = get_string_from_value(engine, "api_key")?;

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

fn get_string_from_value(value: &Value, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    match value.get_str(key) {
        Some(value) => match value.as_str() {
            Some(value) => Ok(value.to_string()),
            None => Err(format!("{} is not a string", key))?,
        },
        None => Err(format!("{} not found in config", key))?,
    }
}

fn get_bool_from_value(value: &Value, key: &str) -> Result<bool, Box<dyn std::error::Error>> {
    match value.get_str(key) {
        Some(value) => match value.as_bool() {
            Some(value) => Ok(value),
            None => Err(format!("{} is not a bool", key))?,
        },
        None => Err(format!("{} not found in config", key))?,
    }
}

fn get_u16_from_value(value: &Value, key: &str) -> Result<u16, Box<dyn std::error::Error>> {
    match value.get_str(key) {
        Some(value) => match value.as_int() {
            Some(value) => Ok(value as u16),
            None => Err(format!("{} is not an int", key))?,
        },
        None => Err(format!("{} not found in config", key))?,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::config::MLEngineType;
    use crate::data::defaults::create_default_config_data;

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
    system_prompt: "
    You are a helpful assistant.
    Answer the question as concisely as possible.
    Do not make up answers.
    "
    enabled: false
  llm:
    model: "qwen3-vl:8b"
    engine_name: my-ollama
    system_prompts:
      easy_language: "
      You are a helpful assistant.
      Translate the following text into easy language.
      Do not include any explanations or additional text.
      "
      very_easy_language: "
      You are a helpful assistant.
      Translate the following text into very easy language.
      Do not include any explanations or additional text.
      "
      summarize: "
      You are a helpful assistant.
      Summarize the following text.
      Do not include any explanations or additional text.
      "
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
        assert_eq!(
            config_data.pipeline_configs.ocr.system_prompt,
            r#"
    You are a helpful assistant.
    Answer the question as concisely as possible.
    Do not make up answers.
    "#
        );
        assert_eq!(config_data.pipeline_configs.ocr.enabled, false);

        assert_eq!(config_data.pipeline_configs.llm.model, "qwen3-vl:8b");
        assert_eq!(config_data.pipeline_configs.llm.vision_model, true);
        assert_eq!(config_data.pipeline_configs.llm.engine_name, "my-ollama");
        assert_eq!(
            config_data
                .pipeline_configs
                .llm
                .system_prompts
                .easy_language,
            r#"
      You are a helpful assistant.
      Translate the following text into easy language.
      Do not include any explanations or additional text.
      "#
        );
        assert_eq!(
            config_data
                .pipeline_configs
                .llm
                .system_prompts
                .very_easy_language,
            r#"
      You are a helpful assistant.
      Translate the following text into very easy language.
      Do not include any explanations or additional text.
      "#
        );
        assert_eq!(
            config_data.pipeline_configs.llm.system_prompts.summarize,
            r#"
      You are a helpful assistant.
      Summarize the following text.
      Do not include any explanations or additional text.
      "#
        );
        assert_eq!(config_data.pipeline_configs.llm.enabled, true);

        assert_eq!(config_data.pipeline_configs.tts.model, "qwen3-tts");
        assert_eq!(config_data.pipeline_configs.tts.engine_name, "my-openai");
        assert_eq!(config_data.pipeline_configs.tts.enabled, true);

        Ok(())
    }
}
