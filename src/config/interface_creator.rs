use crate::data::config::{ConfigData, InterfaceConfig, MLEngineType};
use crate::data::defaults::OLLAMA_DEFAULT_PORT;
use crate::ml_engines::interfaces::llm_interface::LlmInterface;
use crate::ml_engines::interfaces::ocr_interface::OcrInterface;
use crate::ml_engines::interfaces::stt_interface::SttInterface;
use crate::ml_engines::interfaces::tts_interface::TtsInterface;
use crate::ml_engines::ollama::ollama_llm_adapter::OllamaLlmAdapter;
use crate::ml_engines::ollama::ollama_ocr_adapter::OllamaOcrAdapter;
use crate::ml_engines::openai::openai_llm_adapter::OpenAiLlmAdapter;
use crate::ml_engines::openai::openai_ocr_adapter::OpenAiOcrAdapter;
use crate::ml_engines::openai::openai_stt_adapter::OpenAiSttAdapter;
use crate::ml_engines::openai::openai_tts_adapter::OpenAiTtsAdapter;
use async_openai::{Client, config::OpenAIConfig};
use ollama_rs::Ollama;
use std::collections::HashMap;
use std::sync::Arc;
use url::Url;

enum Engines {
    Ollama(Ollama),
    OpenAI(Client<OpenAIConfig>),
}

pub fn create_interfaces(
    config_data: &ConfigData,
) -> Result<InterfaceConfig, Box<dyn std::error::Error>> {
    let used_engine_names = vec![
        config_data.pipeline_configs.stt.engine_name.clone(),
        config_data.pipeline_configs.ocr.engine_name.clone(),
        config_data.pipeline_configs.llm.engine_name.clone(),
        config_data.pipeline_configs.tts.engine_name.clone(),
    ];

    let mut engines: HashMap<String, Engines> = HashMap::new();
    for engine_name in used_engine_names {
        let engine_config = match config_data.ml_engines.get(&engine_name) {
            Some(engine_config) => engine_config,
            None => return Err(format!("Engine {} not found in config", engine_name).into()),
        };
        match engine_config.engine_type {
            MLEngineType::Ollama => {
                let url = Url::parse(&engine_config.url)?;
                let host = match url.host_str() {
                    Some(host) => host,
                    None => return Err(format!("Invalid host in url {}", url).into()),
                };
                let port = url.port().unwrap_or(OLLAMA_DEFAULT_PORT);
                let ollama_host = format!("{}://{}", url.scheme(), host);

                engines.insert(engine_name, Engines::Ollama(Ollama::new(ollama_host, port)));
            }
            MLEngineType::OpenAI => {
                let config = OpenAIConfig::new()
                    .with_api_base(engine_config.url.as_str())
                    .with_api_key(engine_config.api_key.as_str());
                let client = Client::with_config(config);

                engines.insert(engine_name, Engines::OpenAI(client));
            }
        };
    }

    let stt_engine_config = &config_data.pipeline_configs.stt;
    let stt_engine_name = &stt_engine_config.engine_name;
    let stt_interface: Arc<dyn SttInterface> = match engines.get(stt_engine_name) {
        Some(engine) => match engine {
            Engines::OpenAI(client) => Arc::new(OpenAiSttAdapter::new(
                client.clone(),
                (*stt_engine_config).clone(),
            )),
            Engines::Ollama(_) => return Err("STT not supported for Ollama engine".into()),
        },
        None => return Err(format!("Specified STT engine {} not found", stt_engine_name).into()),
    };

    let ocr_engine_config = &config_data.pipeline_configs.ocr;
    let ocr_engine_name = &ocr_engine_config.engine_name;
    let ocr_interface: Arc<dyn OcrInterface> = match engines.get(ocr_engine_name) {
        Some(engine) => match engine {
            Engines::Ollama(client) => Arc::new(OllamaOcrAdapter::new(
                client.clone(),
                (*ocr_engine_config).clone(),
            )),
            Engines::OpenAI(client) => Arc::new(OpenAiOcrAdapter::new(
                client.clone(),
                (*ocr_engine_config).clone(),
            )),
        },
        None => {
            return Err(format!("Specified OCR engine {} not found", ocr_engine_name).into());
        }
    };

    let llm_engine_config = &config_data.pipeline_configs.llm;
    let llm_engine_name = &llm_engine_config.engine_name;
    let llm_interface: Arc<dyn LlmInterface> = match engines.get(llm_engine_name) {
        Some(engine) => match engine {
            Engines::Ollama(engine) => Arc::new(OllamaLlmAdapter::new(
                engine.clone(),
                (*llm_engine_config).clone(),
            )),
            Engines::OpenAI(engine) => Arc::new(OpenAiLlmAdapter::new(
                engine.clone(),
                (*llm_engine_config).clone(),
            )),
        },
        None => {
            return Err(format!("Specified LLM engine {} not found", llm_engine_name).into());
        }
    };

    let tts_engine_config = &config_data.pipeline_configs.tts;
    let tts_engine_name = &tts_engine_config.engine_name;
    let tts_interface: Arc<dyn TtsInterface> = match engines.get(tts_engine_name) {
        Some(engine) => match engine {
            Engines::OpenAI(client) => Arc::new(OpenAiTtsAdapter::new(
                client.clone(),
                (*tts_engine_config).clone(),
            )),
            Engines::Ollama(_) => return Err("TTS not supported for Ollama engine".into()),
        },
        None => return Err(format!("Specified TTS engine {} not found", tts_engine_name).into()),
    };

    Ok(InterfaceConfig {
        stt_interface,
        ocr_interface,
        llm_interface,
        tts_interface,
    })
}
