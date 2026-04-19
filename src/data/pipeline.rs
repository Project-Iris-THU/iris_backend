use crate::data::web::websocket::RequestOpCodes;
use actix_ws::AggregatedMessage;

pub enum PipelineInputData {
    RequestOpCodes(RequestOpCodes),
    AggregatedMessage(AggregatedMessage),
}
