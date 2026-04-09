use std::env;
use log::{debug, info};
use crate::config::data::{ConfigData, LlmConfig, MLEngineConfig, MLEngineType, OcrConfig, SttConfig, TlsConfig};

pub fn load_environment<'config_data>(
    config_data: &'config_data mut ConfigData,
) -> Result<&'config_data mut ConfigData, Box<dyn std::error::Error>> {
    info!("Starting to load environment file:");

    match env::var("IRIS_HOST") {
        Ok(host) => config_data.host = host,
        Err(e) => debug!("Host not found in environment variables: {}", e),
    };

    match env::var("PORT") {
        Ok(port) => config_data.port = port.parse::<u16>()?,
        Err(e) => debug!("Port not found in environment variables: {}", e),
    }


    match env::var("IRIS_TLS_ENABLED") {
        Ok(enabled) => config_data.tls.enabled = enabled.parse::<bool>()?,
        Err(e) => debug!("TLS Enabled not found in environment variables: {}", e),
    };

    match env::var("IRIS_TLS_CERT_FILE") {
        Ok(cert_path) => config_data.tls.cert_path = cert_path,
        Err(e) => debug!("TLS Cert Path not found in environment variables: {}", e),
    };

    match env::var("IRIS_TLS_KEY_FILE") {
        Ok(key_path) => config_data.tls.key_path = key_path,
        Err(e) => debug!("TLS Key Path not found in environment variables: {}", e),
    };

    let ml_engine_count = match env::var("IRIS_ML_ENGINES_COUNT") {
        Ok(ml_engines_count) => ml_engines_count.parse::<usize>()?,
        Err(e) => {
            debug!("ML Engines Count not found in environment variables: {}", e);
            0
        },
    };

    for engine in 0..ml_engine_count {
        parse_ml_engine(engine, config_data)?;
    }

    // STT pipeline configs
    match env::var("IRIS_PIPELINE_STT_MODEL") {
        Ok(model) => config_data.pipeline_configs.stt.model = model,
        Err(e) => debug!("STT Model not found in environment variables: {}", e),
    };

    match env::var("IRIS_PIPELINE_STT_ENGINE_NAME") {
        Ok(engine_name) => config_data.pipeline_configs.stt.engine_name = engine_name,
        Err(e) => debug!("STT Engine Name not found in environment variables: {}", e),
    };

    match env::var("IRIS_PIPELINE_STT_ENGINE_ENABLED") {
        Ok(enabled) => config_data.pipeline_configs.stt.enabled = enabled.parse::<bool>()?,
        Err(e) => debug!("STT Enabled not found in environment variables: {}", e),
    };

    // OCR pipeline configs
    match env::var("IRIS_PIPELINE_OCR_MODEL") {
        Ok(model) => config_data.pipeline_configs.ocr.model = model,
        Err(e) => debug!("OCR Model not found in environment variables: {}", e),
    };

    match env::var("IRIS_PIPELINE_OCR_ENGINE_NAME") {
        Ok(engine_name) => config_data.pipeline_configs.ocr.engine_name = engine_name,
        Err(e) => debug!("OCR Engine Name not found in environment variables: {}", e),
    };

    match env::var("IRIS_PIPELINE_OCR_ENGINE_ENABLED") {
        Ok(enabled) => config_data.pipeline_configs.ocr.enabled = enabled.parse::<bool>()?,
        Err(e) => debug!("OCR Enabled not found in environment variables: {}", e),
    };

    // LLM pipeline configs
    match env::var("IRIS_PIPELINE_LLM_MODEL") {
        Ok(model) => config_data.pipeline_configs.llm.model = model,
        Err(e) => debug!("LLM Model not found in environment variables: {}", e),
    };

    match env::var("IRIS_PIPELINE_LLM_ENGINE_NAME") {
        Ok(engine_name) => config_data.pipeline_configs.llm.engine_name = engine_name,
        Err(e) => debug!("LLM Engine Name not found in environment variables: {}", e),
    };

    match env::var("IRIS_PIPELINE_LLM_VISION_VISION_MODEL") {
        Ok(vision_model) => config_data.pipeline_configs.llm.vision_model =
            vision_model.parse::<bool>()?,
        Err(e) => debug!("LLM vision model not found in environment variables: {}", e),
    };

    match env::var("IRIS_PIPELINE_LLM_ENGINE_ENABLED") {
        Ok(enabled) => config_data.pipeline_configs.llm.enabled = enabled.parse::<bool>()?,
        Err(e) => debug!("LLM Enabled not found in environment variables: {}", e),
    };

    // TTS pipeline configs
    match env::var("IRIS_PIPELINE_TTS_MODEL") {
        Ok(model) => config_data.pipeline_configs.tts.model = model,
        Err(e) => debug!("TTS Model not found in environment variables: {}", e),
    };

    match env::var("IRIS_PIPELINE_TTS_ENGINE_NAME") {
        Ok(engine_name) => config_data.pipeline_configs.tts.engine_name = engine_name,
        Err(e) => debug!("TTS Engine Name not found in environment variables: {}", e),
    };

    match env::var("IRIS_PIPELINE_TTS_ENGINE_ENABLED") {
        Ok(enabled) => config_data.pipeline_configs.tts.enabled = enabled.parse::<bool>()?,
        Err(e) => debug!("TTS Enabled not found in environment variables: {}", e),
    };


    Ok(config_data)
}

fn parse_ml_engine<'config_data>(
    engine_id: usize,
    config_data: &'config_data mut ConfigData,
) -> Result<&'config_data mut ConfigData, Box<dyn std::error::Error>> {
    let name = match env::var(format!("IRIS_ML_ENGINE_{}_NAME", engine_id)) {
        Ok(name) => name,
        Err(e) => {
            let message =
                format!("ML Engine {} name not found in environment variables: {}", engine_id, e);
            debug!("{}", message);
            Err(message)?
        },
    };

    let engine_config_creation_necessary = !config_data.ml_engines.contains_key(&name);

    let mut engine_type = MLEngineType::OpenAI;
    match env::var(format!("IRIS_ML_ENGINE_{}_TYPE", engine_id)) {
        Ok(this_engine_type) => {
            engine_type = match this_engine_type.as_str() {
                "openai" => MLEngineType::OpenAI,
                "ollama" => MLEngineType::Ollama,
                _ => {
                    let message =
                        format!("ML Engine {}: Unsupported type", engine_id);
                    debug!("{}", message);
                    Err(message)?
                },
            };
            if !engine_config_creation_necessary {
                config_data.ml_engines.get_mut(&name).unwrap().engine_type =
                    engine_type.clone();
            }
        },
        Err(e) => {
            let message =
                format!("ML Engine {} type not found in environment variables: {}", engine_id, e);
            debug!("{}", message);
            if engine_config_creation_necessary {
                Err(message)?
            }
        },
    };

    let mut url = "".to_string();
    match env::var(format!("IRIS_ML_ENGINE_{}_URL", engine_id)) {
        Ok(this_url) => {
            url = this_url;
            if !engine_config_creation_necessary {
                config_data.ml_engines.get_mut(&name).unwrap().url = url.clone();
            }
        },
        Err(e) => {
            let message =
                format!("ML Engine {} url not found in environment variables: {}", engine_id, e);
            debug!("{}", message);
            if engine_config_creation_necessary {
                Err(message)?
            }
        },
    };


    let mut api_key = "".to_string();
    match env::var(format!("IRIS_ML_ENGINE_{}_API_KEY", engine_id)) {
        Ok(this_api_key) => {
            api_key = this_api_key;
            if !engine_config_creation_necessary {
                config_data.ml_engines.get_mut(&name).unwrap().api_key = api_key.clone();
            }
        },
        Err(e) => {
            let message =
                format!("ML Engine {} api key not found in environment variables: {}", engine_id, e);
            debug!("{}", message);
            if engine_config_creation_necessary {
                Err(message)?
            }
        },
    };

    if engine_config_creation_necessary {
        config_data.ml_engines.insert(
            name,
            MLEngineConfig {
                engine_type,
                url,
                api_key,
            },
        );
    }

    Ok(config_data)
}
