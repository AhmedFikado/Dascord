use crate::domain::entities::Message;
use crate::infrastructure::repositories::MessageRepository;
use crate::utils::error::{AppResult};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MockMessageRepository {
    messages: Arc<Mutex<HashMap<String, Message>>>, // message_id -> Message
    next_id: Arc<Mutex<u32>>,
}

impl MockMessageRepository {
    pub fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn with_message(self, message: Message) -> Self {
        {
            let mut messages = self.messages.lock().unwrap();
            let id = format!("msg_{}", messages.len() + 1);
            messages.insert(id, message);
        }
        self
    }
}

#[async_trait]
impl MessageRepository for MockMessageRepository {
    async fn create(&self, mut message: Message) -> AppResult<Message> {
        let mut messages = self.messages.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();

        let id = format!("msg_{}", *next_id);
        *next_id += 1;

        // Simulate MongoDB ObjectId
        message.id = Some(mongodb::bson::oid::ObjectId::new());
        messages.insert(id, message.clone());

        Ok(message)
    }

    async fn find_by_channel(&self, channel_id: &str, _limit: i64) -> AppResult<Vec<Message>> {
        let messages = self.messages.lock().unwrap();
        let channel_messages: Vec<Message> = messages
            .values()
            .filter(|m| m.channel_id == channel_id)
            .cloned()
            .collect();
        Ok(channel_messages)
    }

    async fn delete(&self, message_id: &str) -> AppResult<()> {
        let mut messages = self.messages.lock().unwrap();
        messages.remove(message_id);
        Ok(())
    }
}
