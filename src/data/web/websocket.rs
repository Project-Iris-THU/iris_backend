use serde::Serialize;

#[derive(Serialize)]
pub struct ControlMessageRequest {
    op_code: u16,
    system_prompt: Option<String>,
}

#[derive(Serialize)]
pub struct ControlMessageResponse {
    op_code: u16,
    error_message: Option<String>,
}
