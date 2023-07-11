use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ReponseGPT {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: UsageGPT,
    pub choices: Vec<ChoixGPT>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UsageGPT {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChoixGPT {
    pub message: MessageGPT,
    pub finish_reason: String,
    pub index: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageGPT {
    pub role: String,
    pub content: String,
}

