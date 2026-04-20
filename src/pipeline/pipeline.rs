use crate::data::config::InterfaceConfig;
use std::io::Bytes;

use crate::data::ml_engines::SystemPromptType;
use crate::data::pipeline::PipelineInputData;
use crate::data::web::websocket::{RequestOpCodes, ResponseOpCodes};
use actix_ws::{AggregatedMessage, Message};
use log::debug;
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn run(
    mut rx_in: mpsc::UnboundedReceiver<PipelineInputData>,
    tx_out: mpsc::UnboundedSender<AggregatedMessage>,
    interface_config: Arc<InterfaceConfig>,
) {
    let mut last_op_code: Option<RequestOpCodes> = None;
    let mut system_prompt_type = SystemPromptType::EasyLanguage;
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
                        debug!("Custom prompt: {}", system_prompt);
                        system_prompt_type = SystemPromptType::CustomPrompt(system_prompt);
                    }
                    _ => {
                        last_op_code = Some(op_code);
                    }
                };
            }

            PipelineInputData::AggregatedMessage(msg) => match msg {
                AggregatedMessage::Text(text) => {
                    debug!("Text message received: {}", text);
                    tx_out.send(AggregatedMessage::Text(text)).unwrap();
                }

                AggregatedMessage::Binary(bin) => {
                    let content_type;
                    match &last_op_code {
                        Some(RequestOpCodes::Audio {
                            content_type: _content_type,
                        }) => {
                            todo!("Audio pipeline");
                        }
                        Some(RequestOpCodes::Image {
                            content_type: this_content_type,
                        }) => {
                            match ImageHelper::check_image_content_type(
                                &bin,
                                &this_content_type,
                                &tx_out,
                            ) {
                                Ok(_) => {}
                                Err(e) => {
                                    debug!("{}", e);
                                    continue;
                                }
                            };
                            content_type = this_content_type;
                        }
                        Some(_) => {
                            debug!("Not an audio or image message");
                            continue;
                        }
                        None => {
                            debug!("No message received");
                            continue;
                        }
                    }

                    debug!("Pipeline start");

                    let ocr_result = interface_config
                        .ocr_interface
                        .recognize_text(bin, &content_type)
                        .await
                        .unwrap();

                    debug!("Ocr finished");

                    let llm_result = interface_config
                        .llm_interface
                        .generate_text(ocr_result, system_prompt_type.clone())
                        .await
                        .unwrap();

                    debug!("Llm finished");

                    let tts_result = interface_config
                        .tts_interface
                        .generate_audio(llm_result)
                        .await
                        .unwrap();

                    tx_out
                        .send(AggregatedMessage::Text(
                            serde_json::to_string(&ResponseOpCodes::Audio {
                                content_type: "audio/wav".to_string(),
                                done: true,
                            })
                            .unwrap()
                            .into(),
                        ))
                        .unwrap();

                    tx_out.send(AggregatedMessage::Binary(tts_result)).unwrap();
                }

                AggregatedMessage::Ping(msg) => {
                    tx_out.send(AggregatedMessage::Pong(msg)).unwrap();
                }

                AggregatedMessage::Pong(msg) => {
                    tx_out.send(AggregatedMessage::Ping(msg)).unwrap();
                }

                AggregatedMessage::Close(reason) => {
                    debug!("{:?}", reason);
                    tx_out.send(AggregatedMessage::Close(reason)).unwrap();
                    break;
                }
            },
        }
    }
}

struct ImageHelper;
impl ImageHelper {
    fn is_jpeg(bytes: &[u8]) -> bool {
        bytes.starts_with(&[0xFF, 0xD8, 0xFF])
    }

    fn is_png(bytes: &[u8]) -> bool {
        bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])
    }

    fn is_webp(bytes: &[u8]) -> bool {
        bytes.starts_with(&[0x52, 0x49, 0x46, 0x46])
    }

    fn is_gif(bytes: &[u8]) -> bool {
        bytes.starts_with(&[0x47, 0x49, 0x46, 0x38])
    }

    fn is_heic(bytes: &[u8]) -> bool {
        bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])
    }

    fn is_heif(bytes: &[u8]) -> bool {
        bytes.starts_with(&[0x49, 0x49, 0x2A, 0x00])
    }

    fn check_image_content_type(
        bytes: &[u8],
        content_type: &String,
        tx_out: &mpsc::UnboundedSender<AggregatedMessage>,
    ) -> Result<(), String> {
        Ok(match content_type.as_str() {
            "image/jpeg" => {
                if !ImageHelper::is_jpeg(&bytes) {
                    let msg = "Image is not jpeg".to_string();
                    debug!("{}", msg);
                    send_error_message(msg.clone(), &tx_out);
                    return Err(msg);
                }
            }
            "image/png" => {
                if !ImageHelper::is_png(&bytes) {
                    let msg = "Image is not png".to_string();
                    debug!("{}", msg);
                    send_error_message(msg.clone(), &tx_out);
                    return Err(msg);
                }
            }
            "image/webp" => {
                if !ImageHelper::is_webp(&bytes) {
                    let msg = "Image is not webp".to_string();
                    debug!("{}", msg);
                    send_error_message(msg.clone(), &tx_out);
                    return Err(msg);
                }
            }
            "image/gif" => {
                if !ImageHelper::is_gif(&bytes) {
                    let msg = "Image is not gif".to_string();
                    debug!("{}", msg);
                    send_error_message(msg.clone(), &tx_out);
                    return Err(msg);
                }
            }
            "image/heic" => {
                if !ImageHelper::is_heic(&bytes) {
                    let msg = "Image is not heic".to_string();
                    debug!("{}", msg);
                    send_error_message(msg.clone(), &tx_out);
                    return Err(msg);
                }
            }
            "image/heif" => {
                if !ImageHelper::is_heif(&bytes) {
                    let msg = "Image is not heif".to_string();
                    debug!("{}", msg);
                    send_error_message(msg.clone(), &tx_out);
                    return Err(msg);
                }
            }
            _ => {
                let msg = format!("Unsupported content type: {}", content_type);
                debug!("{}", msg);
                send_error_message(msg.clone(), &tx_out);
                return Err(msg);
            }
        })
    }
}

fn send_error_message(msg: String, tx_out: &mpsc::UnboundedSender<AggregatedMessage>) {
    let msg_op_code = ResponseOpCodes::Error { error_message: msg };
    tx_out
        .send(AggregatedMessage::Text(
            serde_json::to_string(&msg_op_code).unwrap().into(),
        ))
        .unwrap();
}
