use crate::domain::entities::channel::PrivateChannel;
use crate::domain::entities::user::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub status: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            status: user.status,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PrivateChannelResponse {
    pub id: Uuid,
    pub user1: Uuid,
    pub user2: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub recipient_user: Option<UserInfo>,
}

impl From<PrivateChannel> for PrivateChannelResponse {
    fn from(channel: PrivateChannel) -> Self {
        Self {
            id: channel.id,
            user1: channel.user1,
            user2: channel.user2,
            created_at: channel.created_at,
            recipient_user: None,
        }
    }
}

impl PrivateChannelResponse {
    pub fn with_recipient(channel: PrivateChannel, recipient: Option<User>) -> Self {
        Self {
            id: channel.id,
            user1: channel.user1,
            user2: channel.user2,
            created_at: channel.created_at,
            recipient_user: recipient.map(UserInfo::from),
        }
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    fn make_user(id: Uuid) -> User {
        User {
            id,
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
            language: "fr".to_string(),
            password_hash: "hash".to_string(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        }
    }

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
        assert!(response.recipient_user.is_none());
    }

    #[test]
    fn test_private_channel_response_with_recipient() {
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();
        let channel = PrivateChannel {
            id: Uuid::new_v4(),
            user1: user1_id,
            user2: user2_id,
            created_at: chrono::Utc::now(),
        };
        let recipient = make_user(user2_id);
        let response = PrivateChannelResponse::with_recipient(channel.clone(), Some(recipient));
        assert_eq!(response.id, channel.id);
        assert!(response.recipient_user.is_some());
        let info = response.recipient_user.unwrap();
        assert_eq!(info.id, user2_id);
        assert_eq!(info.username, "test_user");
        assert_eq!(info.status, "ONLINE");
    }

    #[test]
    fn test_private_channel_response_with_no_recipient() {
        let channel = PrivateChannel {
            id: Uuid::new_v4(),
            user1: Uuid::new_v4(),
            user2: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
        };
        let response = PrivateChannelResponse::with_recipient(channel.clone(), None);
        assert!(response.recipient_user.is_none());
    }
}
