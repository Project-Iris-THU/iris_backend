use crate::data::config::InterfaceConfig;
use std::io::Bytes;

use actix_ws::AggregatedMessage;
use async_trait::async_trait;
use log::debug;
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn run(
    mut rx_in: mpsc::UnboundedReceiver<AggregatedMessage>,
    tx_out: mpsc::UnboundedSender<AggregatedMessage>,
    interface_config: Arc<InterfaceConfig>,
) {
    while let Some(msg) = rx_in.recv().await {
        match msg {
            AggregatedMessage::Text(text) => {
                tx_out.send(AggregatedMessage::Text(text)).unwrap();
            }

            AggregatedMessage::Binary(bin) => {
                if !is_jpeg(&bin) {
                    continue;
                }

                let ocr_result = interface_config
                    .ocr_interface
                    .recognize_text(bin)
                    .await
                    .unwrap();

                let llm_result = interface_config
                    .llm_interface
                    .generate_text(ocr_result)
                    .await
                    .unwrap();

                tx_out
                    .send(AggregatedMessage::Text(llm_result.into()))
                    .unwrap();
            }

            AggregatedMessage::Ping(msg) => {
                tx_out.send(AggregatedMessage::Ping(msg)).unwrap();
            }

            AggregatedMessage::Pong(msg) => {
                tx_out.send(AggregatedMessage::Pong(msg)).unwrap();
            }

            AggregatedMessage::Close(reason) => {
                debug!("{:?}", reason);
                tx_out.send(AggregatedMessage::Close(reason)).unwrap();
                break;
            }
        }
    }
}

fn is_jpeg(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0xFF, 0xD8, 0xFF])
}
