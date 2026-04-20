use crate::data::config::LlmSystemPrompts;
use crate::data::ml_engines::SystemPromptType;

pub fn match_system_prompt_type(
    system_prompt_type: SystemPromptType,
    llm_system_prompts: &LlmSystemPrompts,
) -> String {
    match system_prompt_type {
        SystemPromptType::EasyLanguage => llm_system_prompts.easy_language.clone(),
        SystemPromptType::VeryEasyLanguage => llm_system_prompts.very_easy_language.clone(),
        SystemPromptType::Summarize => llm_system_prompts.summarize.clone(),
        SystemPromptType::CustomPrompt(prompt) => prompt,
    }
}
