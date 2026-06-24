use crate::data::config::InterfaceConfig;
use crate::data::ml_engines::SystemPromptType;
use crate::data::pipeline::PipelineInputData;
use crate::data::web::websocket::{RequestOpCodes, ResponseOpCodes};
use crate::helper::audio::AudioHelper;
use crate::helper::image::ImageHelper;
use actix_ws::AggregatedMessage;
use bytes::Bytes;
use log::{debug, error};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc;

const BUFFER_SIZE: usize = 1024;

pub async fn run(
    mut rx_in: mpsc::UnboundedReceiver<PipelineInputData>,
    tx_out: mpsc::UnboundedSender<AggregatedMessage>,
    mut abort_rx: mpsc::UnboundedReceiver<()>,
    interface_config: Arc<InterfaceConfig>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut last_op_code: Option<RequestOpCodes> = None;
    let mut system_prompt_type = SystemPromptType::EasyLanguage;
    let mut stt_handle = None;
    let mut ocr_handle;

    loop {
        let msg = tokio::select! {
            msg = rx_in.recv() => msg,
            _ = abort_rx.recv() => {
                debug!("Pipeline abort received");
                continue;
            }
        };

        let msg = match msg {
            Some(msg) => msg,
            None => break,
        };

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

                    let transcription_text = match stt_handle.take() {
                        Some(h) => {
                            tokio::select! {
                                result = h => result??,
                                _ = abort_rx.recv() => {
                                    continue;
                                }
                            }
                        }
                        None => String::from(""),
                    };
                    debug!("STT finished");

                    let ocr_text = match ocr_handle.take() {
                        Some(h) => {
                            tokio::select! {
                                result = h => result??,
                                _ = abort_rx.recv() => {
                                    continue;
                                }
                            }
                        }
                        None => String::from(""),
                    };
                    debug!("OCR finished");

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

                    let (tts_chunk_in_channel, mut tts_chunk_out_channel) =
                        mpsc::channel::<String>(BUFFER_SIZE);

                    let tx_out_clone = tx_out.clone();
                    tokio::spawn(async move {
                        let mut sentence_buffer = String::new();

                        while let Some(chunk) = tts_in_channel.recv().await {
                            sentence_buffer.push_str(&chunk);
                            if let Err(send_error) = send_text(&tx_out_clone, &chunk, false) {
                                error!("{send_error}");
                            };

                            if chunk.contains(['.', '!', '?']) {
                                let sentence = sentence_buffer.trim().to_string();
                                if !sentence.is_empty() {
                                    if let Err(send_error) = tts_chunk_in_channel.send(chunk).await
                                    {
                                        error!("{send_error}");
                                    };

                                    sentence_buffer.clear();
                                }
                            }
                        }

                        if let Err(send_error) = send_text(&tx_out_clone, "", true) {
                            error!("{send_error}");
                        };

                        if !sentence_buffer.is_empty() {
                            if let Err(send_error) =
                                tts_chunk_in_channel.send(sentence_buffer).await
                            {
                                error!("{send_error}");
                            };
                        }
                    });

                    let (tts_bytes_stream_channel_tx, mut tts_bytes_stream_channel_rx) =
                        mpsc::channel::<Bytes>(BUFFER_SIZE);

                    let tts_interface_clone = interface_config.tts_interface.clone();
                    tokio::spawn(async move {
                        while let Some(chunk) = tts_chunk_out_channel.recv().await {
                            if let Err(generation_error) = tts_interface_clone
                                .generate_audio_stream(chunk, tts_bytes_stream_channel_tx.clone())
                                .await
                            {
                                error!("{generation_error}");
                            };
                        }
                    });

                    while let Some(audio_bytes_steam) = tts_bytes_stream_channel_rx.recv().await {
                        send_audio(&tx_out, Some(audio_bytes_steam), false)?;
                    }

                    send_audio(&tx_out, None, true)?;

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

fn send_audio(
    tx_out: &mpsc::UnboundedSender<AggregatedMessage>,
    audio: Option<Bytes>,
    done: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tx_out.send(AggregatedMessage::Text(
        serde_json::to_string(&ResponseOpCodes::Audio {
            content_type: "audio/wav".to_string(),
            done,
        })?
        .into(),
    ))?;

    if let Some(audio_data) = audio {
        tx_out.send(AggregatedMessage::Binary(audio_data))?;
    }

    Ok(())
}

fn send_text(
    tx_out: &mpsc::UnboundedSender<AggregatedMessage>,
    text: &str,
    done: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tx_out.send(AggregatedMessage::Text(
        serde_json::to_string(&ResponseOpCodes::Text {
            text: text.to_owned(),
            done,
        })?
        .into(),
    ))?;

    Ok(())
}
