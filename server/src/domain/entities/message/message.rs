use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub channel_id: String,
    pub user_id: String,
    pub username: String,
    pub content: String,
    pub reactions: IndexMap<String, Vec<String>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Message {
    pub fn new(channel_id: String, user_id: String, username: String, content: String) -> Self {
        Self {
            id: None,
            channel_id,
            user_id,
            username,
            content,
            reactions: IndexMap::new(),
            created_at: chrono::Utc::now(),
        }
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let channel_id = "channel123".to_string();
        let user_id = "user123".to_string();
        let username = "Test User".to_string();
        let content = "Hello, World!".to_string();
        let message = Message::new(
            channel_id.clone(),
            user_id.clone(),
            username.clone(),
            content.clone(),
        );
        assert_eq!(message.channel_id, channel_id);
        assert_eq!(message.user_id, user_id);
        assert_eq!(message.username, username);
        assert_eq!(message.content, content);
    }

    #[test]
    fn test_message_id_is_none_on_creation() {
        let message = Message::new(
            "channel".to_string(),
            "user".to_string(),
            "username".to_string(),
            "content".to_string(),
            );
        assert!(message.id.is_none());
    }
}
