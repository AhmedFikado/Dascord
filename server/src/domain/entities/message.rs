use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub channel_id: String,
    pub user_id: String,
    pub username: String,
    pub content: String,
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
            created_at: chrono::Utc::now(),
        }
    }
}
