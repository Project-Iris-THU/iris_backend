use crate::data::config::InterfaceConfig;
use crate::data::ml_engines::SystemPromptType;
use crate::data::pipeline::PipelineInputData;
use crate::data::web::websocket::{RequestOpCodes, ResponseOpCodes};
use crate::helper::audio::AudioHelper;
use crate::helper::image::ImageHelper;
use actix_ws::AggregatedMessage;
use log::{debug, error};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc;

const BUFFER_SIZE: usize = 1024;

pub async fn run(
    mut rx_in: mpsc::UnboundedReceiver<PipelineInputData>,
    tx_out: mpsc::UnboundedSender<AggregatedMessage>,
    interface_config: Arc<InterfaceConfig>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut last_op_code: Option<RequestOpCodes> = None;
    let mut system_prompt_type = SystemPromptType::EasyLanguage;
    let mut stt_handle = None;
    let mut ocr_handle;

    while let Some(msg) = rx_in.recv().await {
        match msg {
            PipelineInputData::RequestOpCodes(op_code) => {
                match op_code {
                    RequestOpCodes::EasyLanguage => {
                        debug!("Easy language");
                        system_prompt_type = SystemPromptType::EasyLanguage;
                    }
                    RequestOpCodes::VeryEasyLanguage => {
                        debug!("Very easy language");
                        system_prompt_type = SystemPromptType::VeryEasyLanguage;
                    }
                    RequestOpCodes::Summarize => {
                        debug!("Summarize");
                        system_prompt_type = SystemPromptType::Summarize;
                    }
                    RequestOpCodes::CustomPrompt { system_prompt } => {
                        debug!("Custom prompt: {system_prompt}");
                        system_prompt_type = SystemPromptType::CustomPrompt(system_prompt);
                    }
                    _ => {
                        last_op_code = Some(op_code);
                    }
                };
            }

            PipelineInputData::AggregatedMessage(msg) => match msg {
                AggregatedMessage::Text(text) => {
                    debug!("Text message received: {text}");
                    tx_out.send(AggregatedMessage::Text(text))?;
                }

                AggregatedMessage::Binary(bin) => {
                    match &last_op_code {
                        Some(RequestOpCodes::Audio {
                            content_type: this_content_type,
                        }) => {
                            match AudioHelper::check_audio_content_type(
                                &bin,
                                this_content_type,
                                &tx_out,
                            ) {
                                Ok(_) => {}
                                Err(e) => {
                                    debug!("{e}");
                                    continue;
                                }
                            };

                            let stt_interface_clone = interface_config.stt_interface.clone();
                            stt_handle = Some(tokio::spawn(async move {
                                debug!("STT started");
                                stt_interface_clone.recognize_speech(bin).await
                            }));
                            continue;
                        }
                        Some(RequestOpCodes::Image {
                            content_type: this_content_type,
                        }) => {
                            match ImageHelper::check_image_content_type(
                                &bin,
                                this_content_type,
                                &tx_out,
                            ) {
                                Ok(_) => {}
                                Err(e) => {
                                    debug!("{e}");
                                    continue;
                                }
                            };

                            let ocr_interface_clone = interface_config.ocr_interface.clone();
                            let content_type_clone = this_content_type.clone();
                            ocr_handle = Some(tokio::spawn(async move {
                                debug!("OCR started");
                                ocr_interface_clone
                                    .recognize_text(bin, &content_type_clone)
                                    .await
                            }));
                        }
                        Some(_) => {
                            debug!("Not an audio or image message");
                            continue;
                        }
                        None => {
                            debug!("No message received");
                            continue;
                        }
                    };

                    debug!("Pipeline start");

                    let transcription_text = if let Some(h) = stt_handle {
                        h.await??
                    } else {
                        String::from("")
                    };
                    debug!("STT finished");

                    let ocr_text = if let Some(h) = ocr_handle {
                        h.await??
                    } else {
                        String::from("")
                    };
                    debug!("OCR finished");

                    stt_handle = None;

                    let llm_input_text =
                        format!("Ocr result: {ocr_text}\n User command: {transcription_text}");

                    let (llm_out_channel, mut tts_in_channel) =
                        mpsc::channel::<String>(BUFFER_SIZE);

                    let system_prompt_type_clone = system_prompt_type.clone();
                    let llm_interface_clone = interface_config.llm_interface.clone();
                    tokio::spawn(async move {
                        if let Err(e) = llm_interface_clone
                            .generate_text_stream(
                                llm_input_text,
                                &system_prompt_type_clone,
                                llm_out_channel,
                            )
                            .await
                        {
                            error!("{e}");
                        }
                    });

                    debug!("Llm finished");

                    while let Some(chunk) = tts_in_channel.recv().await {
                        let tts_result =
                            interface_config.tts_interface.generate_audio(chunk).await?;

                        tx_out.send(AggregatedMessage::Text(
                            serde_json::to_string(&ResponseOpCodes::Audio {
                                content_type: "audio/wav".to_string(),
                                done: false,
                            })?
                            .into(),
                        ))?;

                        tx_out.send(AggregatedMessage::Binary(tts_result))?;
                    }

                    tx_out.send(AggregatedMessage::Text(
                        serde_json::to_string(&ResponseOpCodes::Audio {
                            content_type: "audio/wav".to_string(),
                            done: true,
                        })?
                        .into(),
                    ))?;

                    debug!("Tts finished");
                }

                AggregatedMessage::Ping(msg) => {
                    tx_out.send(AggregatedMessage::Pong(msg))?;
                }

                AggregatedMessage::Pong(msg) => {
                    tx_out.send(AggregatedMessage::Ping(msg))?;
                }

                AggregatedMessage::Close(reason) => {
                    debug!("{reason:?}");
                    tx_out.send(AggregatedMessage::Close(reason))?;
                    break;
                }
            },
        }
    }

    Ok(())
}
