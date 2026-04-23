use crate::infrastructure::repositories::read_status_repository::{
    ReadStatusRepository, UnreadChannelInfo,
};
use crate::utils::error::AppResult;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct MockReadStatusRepository {
    unread: Arc<Mutex<HashMap<(Uuid, Uuid), (String, bool)>>>,
    members: Arc<Mutex<HashMap<Uuid, HashSet<Uuid>>>>,
}

impl MockReadStatusRepository {
    pub fn new() -> Self {
        Self {
            unread: Arc::new(Mutex::new(HashMap::new())),
            members: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_member(self, channel_id: Uuid, user_id: Uuid) -> Self {
        {
            let mut m = self.members.lock().unwrap();
            m.entry(channel_id).or_default().insert(user_id);
        }
        self
    }
}

#[async_trait]
impl ReadStatusRepository for MockReadStatusRepository {
    async fn add_unread(
        &self,
        user_id: Uuid,
        channel_id: Uuid,
        first_message_id: &str,
        is_private: bool,
    ) -> AppResult<()> {
        let mut map = self.unread.lock().unwrap();
        map.entry((user_id, channel_id))
            .or_insert_with(|| (first_message_id.to_string(), is_private));
        Ok(())
    }

    async fn mark_read(&self, user_id: Uuid, channel_id: Uuid) -> AppResult<()> {
        let mut map = self.unread.lock().unwrap();
        map.remove(&(user_id, channel_id));
        Ok(())
    }

    async fn get_unread_channels_for_server(
        &self,
        user_id: Uuid,
        _server_id: Uuid,
    ) -> AppResult<Vec<UnreadChannelInfo>> {
        let map = self.unread.lock().unwrap();
        let result = map
            .iter()
            .filter(|((uid, _), (_, private))| *uid == user_id && !private)
            .map(|((_, channel_id), (msg_id, is_private))| UnreadChannelInfo {
                channel_id: *channel_id,
                first_unread_message_id: msg_id.clone(),
                is_private: *is_private,
            })
            .collect();
        Ok(result)
    }

    async fn get_unread_private_channels(
        &self,
        user_id: Uuid,
    ) -> AppResult<Vec<UnreadChannelInfo>> {
        let map = self.unread.lock().unwrap();
        let result = map
            .iter()
            .filter(|((uid, _), (_, private))| *uid == user_id && *private)
            .map(|((_, channel_id), (msg_id, is_private))| UnreadChannelInfo {
                channel_id: *channel_id,
                first_unread_message_id: msg_id.clone(),
                is_private: *is_private,
            })
            .collect();
        Ok(result)
    }

    async fn get_server_member_ids_for_channel(
        &self,
        channel_id: Uuid,
    ) -> AppResult<Vec<Uuid>> {
        let m = self.members.lock().unwrap();
        Ok(m.get(&channel_id)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default())
    }
}
