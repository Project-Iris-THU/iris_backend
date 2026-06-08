use crate::data::web::websocket::ResponseOpCodes;
use actix_ws::AggregatedMessage;
use tokio::sync::mpsc;

pub struct AudioHelper;
impl AudioHelper {
    fn is_flac(bytes: &[u8]) -> bool {
        bytes.starts_with(b"fLaC")
    }

    fn is_ogg(bytes: &[u8]) -> bool {
        bytes.starts_with(b"OggS")
    }

    fn is_wav(bytes: &[u8]) -> bool {
        bytes.len() > 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WAVE"
    }

    fn is_mpeg(bytes: &[u8]) -> bool {
        bytes.starts_with(b"ID3")
            || (bytes.len() > 2 && bytes[0] == 0xFF && (bytes[1] & 0xE0) == 0xE0)
    }

    fn is_mp4(bytes: &[u8]) -> bool {
        bytes.len() > 8 && &bytes[4..8] == b"ftyp"
    }

    fn is_webm(bytes: &[u8]) -> bool {
        bytes.starts_with(&[0x1A, 0x45, 0xDF, 0xA3])
    }

    pub fn check_audio_content_type(
        bytes: &[u8],
        content_type: &String,
        tx_out: &mpsc::UnboundedSender<AggregatedMessage>,
    ) -> Result<(), String> {
        match content_type.as_str() {
            "audio/flac" => {
                if !AudioHelper::is_flac(bytes) {
                    let msg = "Audio is not flac".to_string();
                    send_error_message(msg.clone(), tx_out);
                    return Err(msg);
                }
            }
            "audio/mpeg" => {
                if !AudioHelper::is_mpeg(bytes) {
                    let msg = "Audio is not mpeg".to_string();
                    send_error_message(msg.clone(), tx_out);
                    return Err(msg);
                }
            }
            "audio/mp4" => {
                if !AudioHelper::is_mp4(bytes) {
                    let msg = "Audio is not mp4".to_string();
                    send_error_message(msg.clone(), tx_out);
                    return Err(msg);
                }
            }
            "audio/ogg" => {
                if !AudioHelper::is_ogg(bytes) {
                    let msg = "Audio is not ogg".to_string();
                    send_error_message(msg.clone(), tx_out);
                    return Err(msg);
                }
            }
            "audio/wav" => {
                if !AudioHelper::is_wav(bytes) {
                    let msg = "Audio is not wav".to_string();
                    send_error_message(msg.clone(), tx_out);
                    return Err(msg);
                }
            }
            "audio/webm" => {
                if !AudioHelper::is_webm(bytes) {
                    let msg = "Audio is not webm".to_string();
                    send_error_message(msg.clone(), tx_out);
                    return Err(msg);
                }
            }
            _ => {
                let msg = format!("Unsupported audio content type: {content_type}");
                send_error_message(msg.clone(), tx_out);
                return Err(msg);
            }
        }
        Ok(())
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
