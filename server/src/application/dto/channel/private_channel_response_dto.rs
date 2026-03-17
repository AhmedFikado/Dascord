use crate::domain::entities::channel::PrivateChannel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PrivateChannelResponse {
    pub id: Uuid,
    pub user1: Uuid,
    pub user2: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<PrivateChannel> for PrivateChannelResponse {
    fn from(channel: PrivateChannel) -> Self {
        Self {
            id: channel.id,
            user1: channel.user1,
            user2: channel.user2,
            created_at: channel.created_at,
        }
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_channel_response_from_entity() {
        let channel = PrivateChannel {
            id: Uuid::new_v4(),
            user1: Uuid::new_v4(),
            user2: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
        };
        let response = PrivateChannelResponse::from(channel.clone());
        assert_eq!(response.id, channel.id);
        assert_eq!(response.user1, channel.user1);
        assert_eq!(response.user2, channel.user2);
        assert_eq!(response.created_at, channel.created_at);
    }
}