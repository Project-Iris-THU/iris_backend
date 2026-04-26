use crate::data::app_state::AppState;
use crate::data::pipeline::PipelineInputData;
use crate::data::web::websocket::{RequestOpCodes, ResponseOpCodes};
use crate::pipeline::run;
use actix_web::{Error, HttpRequest, HttpResponse, get, rt, web};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
use log::debug;
use tokio::sync::mpsc;

#[get("/ws")]
async fn websocket_handler(
    data: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let (tx_in, rx_in) = mpsc::unbounded_channel::<PipelineInputData>();

    let (tx_out, mut rx_out) = mpsc::unbounded_channel::<AggregatedMessage>();

    tokio::spawn(async move { run(rx_in, tx_out, data.interfaces.clone()).await });

    let mut stream = stream
        .max_frame_size(2_usize.pow(25))
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(25));

    let mut session_clone = session.clone();

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            let msg = match msg {
                Ok(msg) => msg,
                Err(e) => {
                    debug!("Here: {e:?}");
                    continue;
                }
            };

            match msg {
                AggregatedMessage::Ping(msg) => {
                    session_clone.pong(&msg).await.unwrap();
                }
                AggregatedMessage::Pong(msg) => {
                    session_clone.ping(&msg).await.unwrap();
                }
                AggregatedMessage::Text(text) => {
                    let data = serde_json::from_str::<RequestOpCodes>(&text);

                    match data {
                        Ok(data) => {
                            match data {
                                RequestOpCodes::AbortPipeline => {
                                    todo!("Abort pipeline");
                                }
                                _ => {
                                    tx_in.send(PipelineInputData::RequestOpCodes(data)).unwrap();
                                }
                            };
                        }
                        Err(e) => {
                            let error_msg = ResponseOpCodes::Error {
                                error_message: format!("Invalid json: {e}"),
                            };
                            session_clone
                                .text(serde_json::to_string(&error_msg).unwrap())
                                .await
                                .unwrap();
                            debug!("{e}");
                        }
                    }
                }
                _ => match tx_in.send(PipelineInputData::AggregatedMessage(msg)) {
                    Ok(_) => (),
                    Err(e) => {
                        debug!("{e}");
                    }
                },
            }
        }
    });

    rt::spawn(async move {
        while let Some(msg) = rx_out.recv().await {
            match msg {
                AggregatedMessage::Text(text) => {
                    session.text(text).await.unwrap();
                }
                AggregatedMessage::Binary(bin) => {
                    session.binary(bin).await.unwrap();
                }
                AggregatedMessage::Ping(msg) => {
                    session.pong(&msg).await.unwrap();
                }
                AggregatedMessage::Pong(msg) => {
                    session.ping(&msg).await.unwrap();
                }
                AggregatedMessage::Close(reason) => {
                    session.close(reason).await.unwrap();
                    break;
                }
            };
        }
    });

    Ok(res)
}
