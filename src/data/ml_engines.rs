#[derive(Clone)]
pub enum SystemPromptType {
    EasyLanguage,
    VeryEasyLanguage,
    Summarize,
    CustomPrompt(String),
}
