use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "op_code", rename_all = "snake_case")]
pub enum RequestOpCodes {
    CustomPrompt { system_prompt: String },
    EasyLanguage,
    VeryEasyLanguage,
    Summarize,
    Image { content_type: String },
    Audio { content_type: String },
    AbortPipeline,
}

#[derive(Serialize)]
#[serde(tag = "op_code", rename_all = "snake_case")]
pub enum ResponseOpCodes {
    Audio { content_type: String, done: bool },
    Error { error_message: String },
    Text { text: String },
}
