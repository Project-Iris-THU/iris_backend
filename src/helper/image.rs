use crate::data::web::websocket::ResponseOpCodes;
use actix_ws::AggregatedMessage;
use log::debug;
use tokio::sync::mpsc;

pub struct ImageHelper;
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

    pub(crate) fn check_image_content_type(
        bytes: &[u8],
        content_type: &String,
        tx_out: &mpsc::UnboundedSender<AggregatedMessage>,
    ) -> Result<(), String> {
        match content_type.as_str() {
            "image/jpeg" => {
                if !ImageHelper::is_jpeg(bytes) {
                    let msg = "Image is not jpeg".to_string();
                    debug!("{msg}");
                    send_error_message(msg.clone(), tx_out);
                    return Err(msg);
                }
            }
            "image/png" => {
                if !ImageHelper::is_png(bytes) {
                    let msg = "Image is not png".to_string();
                    debug!("{msg}");
                    send_error_message(msg.clone(), tx_out);
                    return Err(msg);
                }
            }
            "image/webp" => {
                if !ImageHelper::is_webp(bytes) {
                    let msg = "Image is not webp".to_string();
                    debug!("{msg}");
                    send_error_message(msg.clone(), tx_out);
                    return Err(msg);
                }
            }
            "image/gif" => {
                if !ImageHelper::is_gif(bytes) {
                    let msg = "Image is not gif".to_string();
                    debug!("{msg}");
                    send_error_message(msg.clone(), tx_out);
                    return Err(msg);
                }
            }
            "image/heic" => {
                if !ImageHelper::is_heic(bytes) {
                    let msg = "Image is not heic".to_string();
                    debug!("{msg}");
                    send_error_message(msg.clone(), tx_out);
                    return Err(msg);
                }
            }
            "image/heif" => {
                if !ImageHelper::is_heif(bytes) {
                    let msg = "Image is not heif".to_string();
                    debug!("{msg}");
                    send_error_message(msg.clone(), tx_out);
                    return Err(msg);
                }
            }
            _ => {
                let msg = format!("Unsupported content type: {content_type}");
                debug!("{msg}");
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
