use crate::domain::entities::message::Message;
use crate::utils::error::{AppError, AppResult};
use mongodb::{bson::doc, Collection};

/// Repository pour gérer les messages dans MongoDB
#[derive(Clone)]
pub struct MongoDBMessageRepository {
    collection: Collection<Message>,
}

impl MongoDBMessageRepository {
    /// Créer une nouvelle instance du repository
    pub fn new(collection: Collection<Message>) -> Self {
        Self { collection }
    }

    /// Sauvegarder un nouveau message dans la base de données
    pub async fn save_message(&self, message: &Message) -> AppResult<String> {
        let result = self
            .collection
            .insert_one(message)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to save message: {}", e)))?;

        let id = result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| {
                AppError::DatabaseError("Failed to get inserted message ID".to_string())
            })?
            .to_hex();

        tracing::info!("Message saved with ID: {}", id);
        Ok(id)
    }

    /// Récupérer tous les messages d'un channel (triés par date de création)
    pub async fn get_messages_by_channel(&self, channel_id: &str) -> AppResult<Vec<Message>> {
        let filter = doc! { "channel_id": channel_id };
        let mut cursor = self
            .collection
            .find(filter)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to fetch messages: {}", e)))?;

        let mut messages = Vec::new();
        use futures_util::stream::StreamExt;

        while let Some(result) = cursor.next().await {
            match result {
                Ok(message) => messages.push(message),
                Err(e) => {
                    tracing::error!("Error reading message: {}", e);
                }
            }
        }

        // Trier par date de création (les plus anciens en premier)
        messages.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        tracing::info!(
            "Retrieved {} messages for channel {}",
            messages.len(),
            channel_id
        );
        Ok(messages)
    }

    /// Récupérer les N derniers messages d'un channel
    pub async fn get_recent_messages(
        &self,
        channel_id: &str,
        limit: i64,
    ) -> AppResult<Vec<Message>> {
        use mongodb::options::FindOptions;

        let filter = doc! { "channel_id": channel_id };
        let options = FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .limit(limit)
            .build();

        let mut cursor = self
            .collection
            .find(filter)
            .with_options(options)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to fetch messages: {}", e)))?;

        let mut messages = Vec::new();
        use futures_util::stream::StreamExt;

        while let Some(result) = cursor.next().await {
            match result {
                Ok(message) => messages.push(message),
                Err(e) => {
                    tracing::error!("Error reading message: {}", e);
                }
            }
        }

        // Inverser pour avoir les plus anciens en premier
        messages.reverse();

        tracing::info!(
            "Retrieved {} recent messages for channel {}",
            messages.len(),
            channel_id
        );
        Ok(messages)
    }

    /// Supprimer un message par son ID
    pub async fn delete_message(&self, message_id: &str) -> AppResult<bool> {
        use mongodb::bson::oid::ObjectId;

        let object_id = ObjectId::parse_str(message_id)
            .map_err(|e| AppError::BadRequest(format!("Invalid message ID: {}", e)))?;

        let filter = doc! { "_id": object_id };
        let result = self
            .collection
            .delete_one(filter)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to delete message: {}", e)))?;

        if result.deleted_count > 0 {
            tracing::info!("Message {} deleted successfully", message_id);
            Ok(true)
        } else {
            tracing::warn!("Message {} not found", message_id);
            Ok(false)
        }
    }

    /// Compter le nombre de messages dans un channel
    pub async fn count_messages(&self, channel_id: &str) -> AppResult<u64> {
        let filter = doc! { "channel_id": channel_id };
        let count = self
            .collection
            .count_documents(filter)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to count messages: {}", e)))?;

        Ok(count)
    }
}
