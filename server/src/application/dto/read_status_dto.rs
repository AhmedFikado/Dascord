use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UnreadChannelDto {
    pub channel_id: Uuid,
    pub first_unread_message_id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UnreadStatusResponse {
    pub unread_channels: Vec<UnreadChannelDto>,
}
