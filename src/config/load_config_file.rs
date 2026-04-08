use crate::config::data::{ConfigData, LlmConfig, MLEngineConfig, MLEngineType, OcrConfig, SttConfig};
use rust_yaml::{Value, Yaml};

pub fn load_config_file<'config_data>(
    file_path: &str,
    config_data: &'config_data mut ConfigData,
) -> Result<&'config_data mut ConfigData, Box<dyn std::error::Error>> {
    let yaml = Yaml::new();
    let reader = match std::fs::File::open(file_path) {
        Ok(file) => file,
        Err(e) => Err(e)?,
    };

    let parsed_data = match yaml.load(reader) {
        Ok(data) => data,
        Err(e) => Err(e)?,
    };

    match parsed_data.get_str("host") {
        Some(host) => config_data.host = host.to_string(),
        None => Err("Host not found in config file")?,
    };

    match parsed_data.get_str("port") {
        Some(port) => config_data.port = port.to_string().parse::<u16>()?,
        None => Err("Port not found in config file")?,
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
                Some(model) => model.to_string(),
                None => Err("STT Model not found in config file")?,
            };

            let engine_name = match stt_config.get_str("engine_name") {
                Some(engine_name) => engine_name.to_string(),
                None => Err("STT Engine Name not found in config file")?,
            };

            let enabled = match stt_config.get_str("enabled") {
                Some(enabled) => enabled.to_string().parse::<bool>()?,
                None => Err("STT Enabled not found in config file")?,
            };
            config_data.pipeline_configs.stt = SttConfig { model, engine_name, enabled };
        },
        None => Err("STT Config not found in config file")?,
    };

    match pipeline_configs.get_str("ocr") {
        Some(ocr_config) => {
            let model = match ocr_config.get_str("model") {
                Some(model) => model.to_string(),
                None => Err("OCR Model not found in config file")?,
            };

            let engine_name = match ocr_config.get_str("engine_name") {
                Some(engine_name) => engine_name.to_string(),
                None => Err("OCR Engine Name not found in config file")?,
            };

            let enabled = match ocr_config.get_str("enabled") {
                Some(enabled) => enabled.to_string().parse::<bool>()?,
                None => Err("OCR Enabled not found in config file")?,
            };

            config_data.pipeline_configs.ocr = OcrConfig { model, engine_name, enabled };
        },
        None => Err("OCR Config not found in config file")?,
    };

    match pipeline_configs.get_str("llm") {
        Some(llm_config) => {
            let model = match llm_config.get_str("model") {
                Some(model) => model.to_string(),
                None => Err("STT Model not found in config file")?,
            };

            let engine_name = match llm_config.get_str("engine_name") {
                Some(engine_name) => engine_name.to_string(),
                None => Err("STT Engine Name not found in config file")?,
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
        },
        None => Err("LLM Config not found in config file")?,
    };

    match pipeline_configs.get_str("tts") {
        Some(tts_config) => {
            let model = match tts_config.get_str("model") {
                Some(model) => model.to_string(),
                None => Err("TTS Model not found in config file")?,
            };

            let engine_name = match tts_config.get_str("engine_name") {
                Some(engine_name) => engine_name.to_string(),
                None => Err("TTS Engine Name not found in config file")?,
            };

            let enabled = match tts_config.get_str("enabled") {
                Some(enabled) => enabled.to_string().parse::<bool>()?,
                None => Err("TTS Enabled not found in config file")?,
            };
            config_data.pipeline_configs.stt = SttConfig { model, engine_name, enabled };
        },
        None => Err("TTS Config not found in config file")?,
    };


    Ok(config_data)
}

fn parse_ml_engine<'config_data>(
    engine: &Value,
    config_data: &'config_data mut ConfigData,
) -> Result<&'config_data mut ConfigData, Box<dyn std::error::Error>> {
    let name = match engine.get_str("name") {
        Some(name) => name.to_string(),
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
        Some(url) => url.to_string(),
        None => Err("URL not found in ML Engine config")?,
    };

    let api_key = match engine.get_str("api_key") {
        Some(api_key) => api_key.to_string(),
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
