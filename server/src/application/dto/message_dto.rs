use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageDto {
    pub id: Option<String>,
    pub channel_id: String,
    pub user_id: String,
    pub username: String,
    pub content: String,
    pub reactions: HashMap<String, Vec<String>>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMessageDto {
    pub channel_id: String,
    pub user_id: String,
    pub username: String,
    pub content: String,
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_dto_creation() {
        let dto = MessageDto {
            id: Some("123".to_string()),
            channel_id: "channel1".to_string(),
            user_id: "user1".to_string(),
            username: "testuser".to_string(),
            content: "Hello".to_string(),
            reactions: HashMap::new(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        };
        assert_eq!(dto.content, "Hello");
    }

    #[test]
    fn test_create_message_dto() {
        let dto = CreateMessageDto {
            channel_id: "channel1".to_string(),
            user_id: "user1".to_string(),
            username: "testuser".to_string(),
            content: "Test message".to_string(),
        };
        assert_eq!(dto.username, "testuser");
    }
}
