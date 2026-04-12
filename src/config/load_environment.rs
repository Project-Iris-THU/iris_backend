use crate::data::config::{ConfigData, MLEngineConfig, MLEngineType};
use log::{debug, info};
use std::env;

pub fn load_environment<'config_data>(
    config_data: &'config_data mut ConfigData,
) -> Result<&'config_data mut ConfigData, Box<dyn std::error::Error>> {
    info!("Starting to load environment file:");

    set_string_from_env("IRIS_HOST", &mut config_data.host)?;

    set_u16_from_env("IRIS_PORT", &mut config_data.port)?;

    set_bool_from_env("IRIS_TLS_ENABLED", &mut config_data.tls.enabled)?;

    set_string_from_env("IRIS_TLS_KEY_FILE", &mut config_data.tls.key_path)?;

    set_string_from_env("IRIS_TLS_CERT_FILE", &mut config_data.tls.cert_path)?;

    let ml_engine_count = get_usize_from_env("IRIS_ML_ENGINES_COUNT", false)?;

    for engine in 0..ml_engine_count {
        parse_ml_engine(engine, config_data)?;
    }

    // STT pipeline configs
    set_string_from_env(
        "IRIS_PIPELINE_STT_MODEL",
        &mut config_data.pipeline_configs.stt.model,
    )?;

    set_string_from_env(
        "IRIS_PIPELINE_STT_ENGINE_NAME",
        &mut config_data.pipeline_configs.stt.engine_name,
    )?;

    set_bool_from_env(
        "IRIS_PIPELINE_STT_ENGINE_ENABLED",
        &mut config_data.pipeline_configs.stt.enabled,
    )?;

    // OCR pipeline configs
    set_string_from_env(
        "IRIS_PIPELINE_OCR_MODEL",
        &mut config_data.pipeline_configs.ocr.model,
    )?;

    set_string_from_env(
        "IRIS_PIPELINE_OCR_ENGINE_NAME",
        &mut config_data.pipeline_configs.ocr.engine_name,
    )?;

    set_string_from_env(
        "IRIS_PIPELINE_OCR_SYSTEM_PROMPT",
        &mut config_data.pipeline_configs.ocr.system_prompt,
    )?;

    set_bool_from_env(
        "IRIS_PIPELINE_OCR_ENGINE_ENABLED",
        &mut config_data.pipeline_configs.ocr.enabled,
    )?;

    // LLM pipeline configs
    set_string_from_env(
        "IRIS_PIPELINE_LLM_MODEL",
        &mut config_data.pipeline_configs.llm.model,
    )?;

    set_string_from_env(
        "IRIS_PIPELINE_LLM_ENGINE_NAME",
        &mut config_data.pipeline_configs.llm.engine_name,
    )?;

    set_string_from_env(
        "IRIS_PIPELINE_LLM_SYSTEM_PROMPT",
        &mut config_data.pipeline_configs.llm.system_prompt,
    )?;

    set_bool_from_env(
        "IRIS_PIPELINE_LLM_VISION_MODEL",
        &mut config_data.pipeline_configs.llm.vision_model,
    )?;

    set_bool_from_env(
        "IRIS_PIPELINE_LLM_ENGINE_ENABLED",
        &mut config_data.pipeline_configs.llm.enabled,
    )?;

    // TTS pipeline configs
    set_string_from_env(
        "IRIS_PIPELINE_TTS_MODEL",
        &mut config_data.pipeline_configs.tts.model,
    )?;

    set_string_from_env(
        "IRIS_PIPELINE_TTS_ENGINE_NAME",
        &mut config_data.pipeline_configs.tts.engine_name,
    )?;

    set_bool_from_env(
        "IRIS_PIPELINE_TTS_ENGINE_ENABLED",
        &mut config_data.pipeline_configs.tts.enabled,
    )?;

    Ok(config_data)
}

fn parse_ml_engine<'config_data>(
    engine_id: usize,
    config_data: &'config_data mut ConfigData,
) -> Result<&'config_data mut ConfigData, Box<dyn std::error::Error>> {
    let name = get_string_from_env(format!("IRIS_ML_ENGINE_{}_NAME", engine_id).as_str(), true)?;

    let engine_config_creation_necessary = !config_data.ml_engines.contains_key(&name);
    if engine_config_creation_necessary {
        config_data.ml_engines.insert(
            name.clone(),
            MLEngineConfig {
                engine_type: MLEngineType::OpenAI,
                url: "".to_string(),
                api_key: "".to_string(),
            },
        );
    }

    let engine_type = set_engine_type_if_key_exists(
        format!("IRIS_ML_ENGINE_{}_TYPE", engine_id).as_str(),
        &mut config_data.ml_engines.get_mut(&name).unwrap().engine_type,
        !engine_config_creation_necessary,
    )?;

    let url = set_string_if_key_exists(
        format!("IRIS_ML_ENGINE_{}_URL", engine_id).as_str(),
        &mut config_data.ml_engines.get_mut(&name).unwrap().url,
        !engine_config_creation_necessary,
    )?;

    let api_key = set_string_if_key_exists(
        format!("IRIS_ML_ENGINE_{}_API_KEY", engine_id).as_str(),
        &mut config_data.ml_engines.get_mut(&name).unwrap().api_key,
        !engine_config_creation_necessary,
    )?;

    Ok(config_data)
}

fn set_string_from_env(
    env_var_name: &str,
    destination: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    match env::var(env_var_name) {
        Ok(value) => {
            *destination = value;
            Ok(())
        }
        Err(e) => {
            let message = format!("{} not found in environment variables: {}", env_var_name, e);
            debug!("{}", message);
            Ok(())
        }
    }
}

fn set_string_if_key_exists(
    env_var_name: &str,
    destination: &mut String,
    key_exists: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match env::var(env_var_name) {
        Ok(value) => {
            *destination = value;
            Ok(())
        }
        Err(e) => {
            let message = format!("{} not found in environment variables: {}", env_var_name, e);
            if key_exists {
                debug!("{}", message);
                Ok(())
            } else {
                Err(message)?
            }
        }
    }
}

fn get_string_from_env(
    env_var_name: &str,
    mandatory: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    match env::var(env_var_name) {
        Ok(value) => Ok(value),
        Err(e) => {
            let message = format!("{} not found in environment variables: {}", env_var_name, e);
            debug!("{}", message);
            if mandatory {
                Err(message)?
            } else {
                Ok("".to_string())
            }
        }
    }
}

fn set_engine_type_if_key_exists(
    env_var_name: &str,
    destination: &mut MLEngineType,
    key_exists: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let engine_type_string = get_string_from_env(env_var_name, key_exists)?;
    let engine_type = match engine_type_string.as_str() {
        "openai" => MLEngineType::OpenAI,
        "ollama" => MLEngineType::Ollama,
        _ => {
            let message = format!("{}: Unsupported type", env_var_name);
            debug!("{}", message);
            Err(message)?
        }
    };
    *destination = engine_type;
    Ok(())
}

fn set_bool_from_env(
    env_var_name: &str,
    destination: &mut bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match env::var(env_var_name) {
        Ok(value) => {
            *destination = value.parse::<bool>()?;
            Ok(())
        }
        Err(e) => {
            let message = format!("{} not found in environment variables: {}", env_var_name, e);
            debug!("{}", message);
            Ok(())
        }
    }
}

fn set_u16_from_env(
    env_var_name: &str,
    destination: &mut u16,
) -> Result<(), Box<dyn std::error::Error>> {
    match env::var(env_var_name) {
        Ok(value) => {
            *destination = value.parse::<u16>()?;
            Ok(())
        }
        Err(e) => {
            let message = format!("{} not found in environment variables: {}", env_var_name, e);
            debug!("{}", message);
            Ok(())
        }
    }
}

fn get_usize_from_env(
    env_var_name: &str,
    mandatory: bool,
) -> Result<usize, Box<dyn std::error::Error>> {
    match env::var(env_var_name) {
        Ok(value) => Ok(value.parse::<usize>()?),
        Err(e) => {
            let message = format!("{} not found in environment variables: {}", env_var_name, e);
            debug!("{}", message);
            if mandatory { Err(message)? } else { Ok(0) }
        }
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
            env::set_var("IRIS_PIPELINE_LLM_VISION_MODEL", "true");
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
            env::remove_var("IRIS_PIPELINE_LLM_VISION_MODEL");
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
