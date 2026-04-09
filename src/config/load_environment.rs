use crate::config::data::{ConfigData, MLEngineConfig, MLEngineType};
use log::{debug, info};
use std::env;

pub fn load_environment<'config_data>(
    config_data: &'config_data mut ConfigData,
) -> Result<&'config_data mut ConfigData, Box<dyn std::error::Error>> {
    info!("Starting to load environment file:");

    match env::var("IRIS_HOST") {
        Ok(host) => config_data.host = host,
        Err(e) => debug!("Host not found in environment variables: {}", e),
    };

    match env::var("IRIS_PORT") {
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
        }
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
        Ok(vision_model) => {
            config_data.pipeline_configs.llm.vision_model = vision_model.parse::<bool>()?
        }
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
            let message = format!(
                "ML Engine {} name not found in environment variables: {}",
                engine_id, e
            );
            debug!("{}", message);
            Err(message)?
        }
    };

    let engine_config_creation_necessary = !config_data.ml_engines.contains_key(&name);

    let mut engine_type = MLEngineType::OpenAI;
    match env::var(format!("IRIS_ML_ENGINE_{}_TYPE", engine_id)) {
        Ok(this_engine_type) => {
            engine_type = match this_engine_type.as_str() {
                "openai" => MLEngineType::OpenAI,
                "ollama" => MLEngineType::Ollama,
                _ => {
                    let message = format!("ML Engine {}: Unsupported type", engine_id);
                    debug!("{}", message);
                    Err(message)?
                }
            };
            if !engine_config_creation_necessary {
                config_data.ml_engines.get_mut(&name).unwrap().engine_type = engine_type.clone();
            }
        }
        Err(e) => {
            let message = format!(
                "ML Engine {} type not found in environment variables: {}",
                engine_id, e
            );
            debug!("{}", message);
            if engine_config_creation_necessary {
                Err(message)?
            }
        }
    };

    let mut url = "".to_string();
    match env::var(format!("IRIS_ML_ENGINE_{}_URL", engine_id)) {
        Ok(this_url) => {
            url = this_url;
            if !engine_config_creation_necessary {
                config_data.ml_engines.get_mut(&name).unwrap().url = url.clone();
            }
        }
        Err(e) => {
            let message = format!(
                "ML Engine {} url not found in environment variables: {}",
                engine_id, e
            );
            debug!("{}", message);
            if engine_config_creation_necessary {
                Err(message)?
            }
        }
    };

    let mut api_key = "".to_string();
    match env::var(format!("IRIS_ML_ENGINE_{}_API_KEY", engine_id)) {
        Ok(this_api_key) => {
            api_key = this_api_key;
            if !engine_config_creation_necessary {
                config_data.ml_engines.get_mut(&name).unwrap().api_key = api_key.clone();
            }
        }
        Err(e) => {
            let message = format!(
                "ML Engine {} api key not found in environment variables: {}",
                engine_id, e
            );
            debug!("{}", message);
            if engine_config_creation_necessary {
                Err(message)?
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::data::{MLEngineType, create_default_config_data};

    #[test]
    fn test_load_correct_config_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut config_data = create_default_config_data();

        unsafe {
            env::set_var("IRIS_HOST", "0.0.0.0");
            env::set_var("IRIS_PORT", "1234");
            env::set_var("IRIS_TLS_ENABLED", "false");
            env::set_var("IRIS_TLS_KEY_FILE", "some/path/to/privkey.pem");
            env::set_var("IRIS_TLS_CERT_FILE", "some/path/to/fullchain.pem");
            env::set_var("IRIS_ML_ENGINES_COUNT", "2");
            env::set_var("IRIS_ML_ENGINE_0_NAME", "my-openai");
            env::set_var("IRIS_ML_ENGINE_0_TYPE", "openai");
            env::set_var("IRIS_ML_ENGINE_0_URL", "https://api.openai.com/v1/");
            env::set_var("IRIS_ML_ENGINE_0_API_KEY", "openai api key");
            env::set_var("IRIS_ML_ENGINE_1_NAME", "my-ollama");
            env::set_var("IRIS_ML_ENGINE_1_TYPE", "ollama");
            env::set_var("IRIS_ML_ENGINE_1_URL", "http://localhost:11434/");
            env::set_var("IRIS_ML_ENGINE_1_API_KEY", "no api key");
            env::set_var("IRIS_PIPELINE_STT_MODEL", "whisper-large-v3-turbo");
            env::set_var("IRIS_PIPELINE_STT_ENGINE_NAME", "my-openai");
            env::set_var("IRIS_PIPELINE_STT_ENGINE_ENABLED", "true");
            env::set_var("IRIS_PIPELINE_OCR_MODEL", "deepseek-ocr");
            env::set_var("IRIS_PIPELINE_OCR_ENGINE_NAME", "my-ollama");
            env::set_var("IRIS_PIPELINE_OCR_ENGINE_ENABLED", "true");
            env::set_var("IRIS_PIPELINE_LLM_MODEL", "qwen3-vl:8b");
            env::set_var("IRIS_PIPELINE_LLM_ENGINE_NAME", "my-ollama");
            env::set_var("IRIS_PIPELINE_LLM_VISION_VISION_MODEL", "true");
            env::set_var("IRIS_PIPELINE_LLM_ENGINE_ENABLED", "true");
            env::set_var("IRIS_PIPELINE_TTS_MODEL", "qwen3-tts");
            env::set_var("IRIS_PIPELINE_TTS_ENGINE_NAME", "my-openai");
            env::set_var("IRIS_PIPELINE_TTS_ENGINE_ENABLED", "true");
        }

        load_environment(&mut config_data)?;

        unsafe {
            env::remove_var("IRIS_HOST");
            env::remove_var("IRIS_PORT");
            env::remove_var("IRIS_TLS_ENABLED");
            env::remove_var("IRIS_TLS_KEY_FILE");
            env::remove_var("IRIS_TLS_CERT_FILE");
            env::remove_var("IRIS_ML_ENGINES_COUNT");
            env::remove_var("IRIS_ML_ENGINE_0_NAME");
            env::remove_var("IRIS_ML_ENGINE_0_TYPE");
            env::remove_var("IRIS_ML_ENGINE_0_URL");
            env::remove_var("IRIS_ML_ENGINE_0_API_KEY");
            env::remove_var("IRIS_ML_ENGINE_1_NAME");
            env::remove_var("IRIS_ML_ENGINE_1_TYPE");
            env::remove_var("IRIS_ML_ENGINE_1_URL");
            env::remove_var("IRIS_ML_ENGINE_1_API_KEY");
            env::remove_var("IRIS_PIPELINE_STT_MODEL");
            env::remove_var("IRIS_PIPELINE_STT_ENGINE_NAME");
            env::remove_var("IRIS_PIPELINE_STT_ENGINE_ENABLED");
            env::remove_var("IRIS_PIPELINE_OCR_MODEL");
            env::remove_var("IRIS_PIPELINE_OCR_ENGINE_NAME");
            env::remove_var("IRIS_PIPELINE_OCR_ENGINE_ENABLED");
            env::remove_var("IRIS_PIPELINE_LLM_MODEL");
            env::remove_var("IRIS_PIPELINE_LLM_ENGINE_NAME");
            env::remove_var("IRIS_PIPELINE_LLM_VISION_VISION_MODEL");
            env::remove_var("IRIS_PIPELINE_LLM_ENGINE_ENABLED");
            env::remove_var("IRIS_PIPELINE_TTS_MODEL");
            env::remove_var("IRIS_PIPELINE_TTS_ENGINE_NAME");
            env::remove_var("IRIS_PIPELINE_TTS_ENGINE_ENABLED");
        }

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
        assert_eq!(config_data.pipeline_configs.ocr.enabled, true);

        assert_eq!(config_data.pipeline_configs.llm.model, "qwen3-vl:8b");
        assert_eq!(config_data.pipeline_configs.llm.vision_model, true);
        assert_eq!(config_data.pipeline_configs.llm.engine_name, "my-ollama");
        assert_eq!(config_data.pipeline_configs.llm.enabled, true);

        assert_eq!(config_data.pipeline_configs.tts.model, "qwen3-tts");
        assert_eq!(config_data.pipeline_configs.tts.engine_name, "my-openai");
        assert_eq!(config_data.pipeline_configs.tts.enabled, true);

        Ok(())
    }
}
