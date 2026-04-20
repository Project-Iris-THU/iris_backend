use serde::Serialize;

#[derive(Serialize)]
pub struct InfoResponse {
    pub(crate) name: &'static str,
    pub(crate) version: &'static str,
}
