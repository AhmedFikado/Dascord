use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Channel {
    pub id: Uuid,
    pub server_id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Channel {
    pub fn new(server_id: Uuid, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            server_id,
            name,
            created_at: chrono::Utc::now(),
        }
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_creation() {
        let server_id = Uuid::new_v4();
        let name = "General".to_string();

        let channel = Channel::new(server_id, name.clone());

        assert_eq!(channel.server_id, server_id);
        assert_eq!(channel.name, name);
    }

    #[test]
    fn test_channel_has_unique_id() {
        let server_id = Uuid::new_v4();
        let channel1 = Channel::new(server_id, "Channel1".to_string());
        let channel2 = Channel::new(server_id, "Channel2".to_string());

        assert_ne!(channel1.id, channel2.id);
    }
}
