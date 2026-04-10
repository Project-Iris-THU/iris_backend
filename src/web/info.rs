use actix_web::{Responder, get, web};
use serde::Serialize;

#[derive(Serialize)]
struct InfoResponse {
    name: &'static str,
    version: &'static str,
}

#[get("/info")]
pub async fn info() -> Result<impl Responder, actix_web::Error> {
    let response = InfoResponse {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    };

    Ok(web::Json(response))
}
