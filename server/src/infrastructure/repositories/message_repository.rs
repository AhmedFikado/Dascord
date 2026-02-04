use crate::domain::entities::Message;
use crate::utils::error::{AppError, AppResult};
use async_trait::async_trait;
use mongodb::{Client as MongoClient, bson::doc};

#[async_trait]
pub trait MessageRepository: Send + Sync + Clone {
    async fn create(&self, message: Message) -> AppResult<Message>;
    async fn find_by_channel(&self, channel_id: &str) -> AppResult<Vec<Message>>;
    async fn delete(&self, message_id: &str) -> AppResult<()>;
}

#[derive(Clone)]
pub struct MongoMessageRepository {
    client: MongoClient,
}

impl MongoMessageRepository {
    pub fn new(client: MongoClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl MessageRepository for MongoMessageRepository {
    async fn create(&self, message: Message) -> AppResult<Message> {
        let db = self.client.database("discord_db");
        let collection = db.collection::<Message>("messages");
        
        let result = collection.insert_one(&message).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        
        let mut created_message = message;
        created_message.id = result.inserted_id.as_object_id();
        
        Ok(created_message)
    }

    async fn find_by_channel(&self, channel_id: &str) -> AppResult<Vec<Message>> {
        let db = self.client.database("discord_db");
        let collection = db.collection::<Message>("messages");
        
        let filter = doc! { "channel_id": channel_id };
        let options = mongodb::options::FindOptions::builder()
            .sort(doc! { "created_at": 1 })  // Trier par date croissante (plus ancien en premier)
            .build();
        
        let mut cursor = collection.find(filter).with_options(options).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        
        let mut messages = Vec::new();
        use futures_util::StreamExt;
        while let Some(result) = cursor.next().await {
            match result {
                Ok(message) => messages.push(message),
                Err(e) => return Err(AppError::InternalServerError(e.to_string())),
            }
        }
        
        Ok(messages)
    }

    async fn delete(&self, message_id: &str) -> AppResult<()> {
        let db = self.client.database("discord_db");
        let collection = db.collection::<Message>("messages");
        
        let object_id = mongodb::bson::oid::ObjectId::parse_str(message_id)
            .map_err(|_| AppError::ValidationError("Invalid message ID".to_string()))?;
        
        let filter = doc! { "_id": object_id };
        collection.delete_one(filter).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        
        Ok(())
    }
}
