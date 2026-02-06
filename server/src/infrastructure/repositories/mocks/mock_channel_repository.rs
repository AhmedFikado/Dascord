use crate::domain::entities::Channel;
use crate::infrastructure::repositories::ChannelRepository;
use crate::utils::error::{AppResult};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct MockChannelRepository {
    channels: Arc<Mutex<HashMap<Uuid, Channel>>>,
}

impl MockChannelRepository {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_channel(self, channel: Channel) -> Self {
        {
            let mut channels = self.channels.lock().unwrap();
            channels.insert(channel.id, channel);
        }
        self
    }
}

#[async_trait]
impl ChannelRepository for MockChannelRepository {
    async fn create(&self, channel: Channel) -> AppResult<Channel> {
        let mut channels = self.channels.lock().unwrap();
        channels.insert(channel.id, channel.clone());
        Ok(channel)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Channel>> {
        let channels = self.channels.lock().unwrap();
        Ok(channels.get(&id).cloned())
    }

    async fn find_by_server(&self, server_id: Uuid) -> AppResult<Vec<Channel>> {
        let channels = self.channels.lock().unwrap();
        let server_channels: Vec<Channel> = channels
            .values()
            .filter(|c| c.server_id == server_id)
            .cloned()
            .collect();
        Ok(server_channels)
    }

    async fn update(&self, channel: Channel) -> AppResult<Channel> {
        let mut channels = self.channels.lock().unwrap();
        channels.insert(channel.id, channel.clone());
        Ok(channel)
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut channels = self.channels.lock().unwrap();
        channels.remove(&id);
        Ok(())
    }
}
