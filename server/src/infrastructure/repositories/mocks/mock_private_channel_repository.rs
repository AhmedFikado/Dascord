use crate::domain::entities::channel::PrivateChannel;
use crate::infrastructure::repositories::channel::PrivateChannelRepository;
use crate::utils::error::AppResult;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct MockPrivateChannelRepository {
    channels: Arc<Mutex<Vec<PrivateChannel>>>,
}

impl MockPrivateChannelRepository {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn with_channels(channels: Vec<PrivateChannel>) -> Self {
        Self {
            channels: Arc::new(Mutex::new(channels)),
        }
    }
}

impl Default for MockPrivateChannelRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PrivateChannelRepository for MockPrivateChannelRepository {
    async fn create(&self, user1: Uuid, user2: Uuid) -> AppResult<PrivateChannel> {
        let id = Uuid::new_v4();
        let channel = PrivateChannel {
            id,
            user1,
            user2,
            created_at: chrono::Utc::now(),
            user1_hidden: false,
            user2_hidden: false,
        };

        let mut channels = self.channels.lock().unwrap();
        channels.push(channel.clone());

        Ok(channel)
    }

    async fn get_by_id(&self, id: Uuid) -> AppResult<Option<PrivateChannel>> {
        let channels = self.channels.lock().unwrap();
        Ok(channels.iter().find(|c| c.id == id).cloned())
    }

    async fn get_by_users(
        &self,
        user1: Uuid,
        user2: Uuid,
    ) -> AppResult<Option<PrivateChannel>> {
        let channels = self.channels.lock().unwrap();
        Ok(channels
            .iter()
            .find(|c| {
                (c.user1 == user1 && c.user2 == user2) || (c.user1 == user2 && c.user2 == user1)
            })
            .cloned())
    }

    async fn get_user_channels(&self, user_id: Uuid) -> AppResult<Vec<PrivateChannel>> {
        let channels = self.channels.lock().unwrap();
        let mut user_channels: Vec<PrivateChannel> = channels
            .iter()
            .filter(|c| c.user1 == user_id || c.user2 == user_id)
            .cloned()
            .collect();

        user_channels.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(user_channels)
    }

    async fn update_last_message_at(&self, _channel_id: Uuid, _sender_id: Uuid, _last_message_at: chrono::DateTime<chrono::Utc>) -> AppResult<()> {
        Ok(())
    }

    async fn hide_channel(&self, channel_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let mut channels = self.channels.lock().unwrap();
        for channel in channels.iter_mut() {
            if channel.id == channel_id {
                if channel.user1 == user_id {
                    channel.user1_hidden = true;
                } else if channel.user2 == user_id {
                    channel.user2_hidden = true;
                }
                break;
            }
        }
        Ok(())
    }

    async fn unhide_channel(&self, channel_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let mut channels = self.channels.lock().unwrap();
        for channel in channels.iter_mut() {
            if channel.id == channel_id {
                if channel.user1 == user_id {
                    channel.user1_hidden = false;
                } else if channel.user2 == user_id {
                    channel.user2_hidden = false;
                }
                break;
            }
        }
        Ok(())
    }
}