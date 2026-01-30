use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDto {
    pub id: Option<String>,
    pub channel_id: String,
    pub user_id: String,
    pub username: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMessageDto {
    pub channel_id: String,
    pub user_id: String,
    pub username: String,
    pub content: String,
}
