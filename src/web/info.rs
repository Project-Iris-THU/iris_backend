use crate::data::web::info::InfoResponse;
use actix_web::{Responder, get, web};

#[get("/info")]
pub async fn info() -> Result<impl Responder, actix_web::Error> {
    let response = InfoResponse {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    };

    Ok(web::Json(response))
}
