use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::domain::entities::Channel;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateChannelRequest {
    #[validate(length(min = 1, max = 100, message = "Channel name must be between 1 and 100 characters"))]
    pub name: String,
}

#[derive(Debug, Serialize)]
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
