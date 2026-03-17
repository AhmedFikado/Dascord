use crate::domain::entities::channel::Channel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateChannelRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Channel name must be between 1 and 100 characters"
    ))]
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChannelResponse {
    pub id: String,
    pub server_id: String,
    pub name: String,
    pub created_at: String,
}

impl From<Channel> for ChannelResponse {
    fn from(channel: Channel) -> Self {
        Self {
            id: channel.id.to_string(),
            server_id: channel.server_id.to_string(),
            name: channel.name,
            created_at: channel.created_at.to_rfc3339(),
        }
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use validator::Validate;

    #[test]
    fn test_valid_create_channel_request() {
        let request = CreateChannelRequest {
            name: "general".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_empty_channel_name() {
        let request = CreateChannelRequest {
            name: "".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_channel_response_from_entity() {
        let channel = Channel {
            id: Uuid::new_v4(),
            server_id: Uuid::new_v4(),
            name: "test-channel".to_string(),
            created_at: chrono::Utc::now(),
        };
        let response = ChannelResponse::from(channel.clone());
        assert_eq!(response.name, "test-channel");
    }
}
