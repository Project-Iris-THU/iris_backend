use crate::config::data::{ConfigData, InterfaceConfig, MLEngineType};
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
use ollama_rs::Ollama;
use openai_api_rust::Auth;
use openai_api_rust::OpenAI;
use std::collections::HashMap;
use std::sync::Arc;

enum EngineType {
    Ollama(Arc<Ollama>),
    OpenAI(Arc<OpenAI>),
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

    let mut ollama_engines = HashMap::new();
    let mut openai_engines = HashMap::new();
    for engine_name in used_engine_names {
        let engine_config = match config_data.ml_engines.get(&engine_name) {
            Some(engine_config) => engine_config,
            None => return Err(format!("Engine {} not found in config", engine_name).into()),
        };
        match engine_config.engine_type {
            MLEngineType::Ollama => {
                //TODO: Fix ollama initialization
                let url = engine_config.url.clone();
                ollama_engines.insert(
                    engine_name,
                    Arc::new(Ollama::new(engine_config.url.clone(), Default::default())),
                );
            }
            MLEngineType::OpenAI => {
                let auth = Auth::new(engine_config.api_key.as_str());
                openai_engines.insert(
                    engine_name,
                    Arc::new(OpenAI::new(auth, engine_config.url.as_str())),
                );
            }
        };
    }

    let stt_engine_name = &config_data.pipeline_configs.stt.engine_name;
    let stt_interface: Box<dyn SttInterface> = match openai_engines.get(stt_engine_name) {
        Some(engine) => Box::new(OpenAiSttAdapter::new(Arc::clone(engine))),
        None => return Err(format!("Specified engine {} not found", stt_engine_name).into()),
    };

    let ocr_engine_name = &config_data.pipeline_configs.ocr.engine_name;
    let ocr_interface: Box<dyn OcrInterface> = match ollama_engines.get(ocr_engine_name) {
        Some(engine) => Box::new(OllamaOcrAdapter::new(Arc::clone(engine))),
        None => match openai_engines.get(ocr_engine_name) {
            Some(engine) => Box::new(OpenAiOcrAdapter::new(Arc::clone(engine))),
            None => return Err(format!("Specified engine {} not found", ocr_engine_name).into()),
        },
    };

    let llm_engine_name = &config_data.pipeline_configs.llm.engine_name;
    let llm_interface: Box<dyn LlmInterface> = match ollama_engines.get(llm_engine_name) {
        Some(engine) => Box::new(OllamaLlmAdapter::new(Arc::clone(engine))),
        None => match openai_engines.get(llm_engine_name) {
            Some(engine) => Box::new(OpenAiLlmAdapter::new(Arc::clone(engine))),
            None => return Err(format!("Specified engine {} not found", llm_engine_name).into()),
        },
    };

    let tts_engine_name = &config_data.pipeline_configs.tts.engine_name;
    let tts_interface: Box<dyn TtsInterface> = match openai_engines.get(tts_engine_name) {
        Some(engine) => Box::new(OpenAiTtsAdapter::new(Arc::clone(engine))),
        None => return Err(format!("Specified engine {} not found", tts_engine_name).into()),
    };

    Ok(InterfaceConfig {
        stt_interface,
        ocr_interface,
        llm_interface,
        tts_interface,
    })
}
