use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PrivateChannel {
    pub id: Uuid,
    pub user1: Uuid,
    pub user2: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl PrivateChannel {
    pub fn new(user1: Uuid, user2: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user1,
            user2,
            created_at: chrono::Utc::now(),
        }
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_channel_creation() {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();

        let channel = PrivateChannel::new(user1, user2);

        assert_eq!(channel.user1, user1);
        assert_eq!(channel.user2, user2);
    }

    #[test]
    fn test_private_channel_has_unique_id() {
        let user1 = Uuid::new_v4();
        let user2 = Uuid::new_v4();
        let channel1 = PrivateChannel::new(user1, user2);
        let channel2 = PrivateChannel::new(user1, user2);

        assert_ne!(channel1.id, channel2.id);
    }
}