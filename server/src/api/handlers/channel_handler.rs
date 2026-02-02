use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::infrastructure::security::JWTService;
use crate::utils::error::AppError;

/// Handler contenant la logique métier pour les canaux
#[derive(Clone)]
pub struct ChannelHandler {
    jwt_service: Arc<JWTService>,
}

impl ChannelHandler {
    pub fn new(jwt_service: JWTService) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service),
        }
    }

    /// GET /channels/:id - Obtenir les infos d'un canal
    pub async fn get_channel_info(
        State(handler): State<Arc<ChannelHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Récupérer les infos du canal depuis la DB
        // let channel = handler.channel_uc.get_channel_by_id(id).await?;
        // Vérifier que l'utilisateur a accès à ce canal
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "channel_id": id.to_string(),
            "user_id": claims.sub_id,
            "name": "general",
            "server_id": "server-uuid",
            "created_at": "2024-01-01T00:00:00Z"
        }))))
    }

    /// PUT /channels/:id - Mettre à jour un canal
    pub async fn update_channel(
        State(handler): State<Arc<ChannelHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
        Json(payload): Json<serde_json::Value>,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Mettre à jour le canal
        // Vérifier que l'utilisateur est owner/admin du serveur
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "message": "Channel updated",
            "channel_id": id.to_string(),
            "user_id": claims.sub_id
        }))))
    }

    /// DELETE /channels/:id - Supprimer un canal
    pub async fn delete_channel(
        State(handler): State<Arc<ChannelHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Supprimer le canal
        // Vérifier que l'utilisateur est owner/admin du serveur
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "message": "Channel deleted",
            "channel_id": id.to_string()
        }))))
    }

    /// POST /channels/:id/messages - Envoyer un message dans un canal
    pub async fn send_message(
        State(handler): State<Arc<ChannelHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
        Json(payload): Json<serde_json::Value>,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Créer le message
        // let message = handler.message_uc.send_message(id, claims.sub_id, payload).await?;
        // Vérifier que l'utilisateur a accès au canal
        
        Ok((StatusCode::CREATED, Json(serde_json::json!({
            "message": "Message sent",
            "channel_id": id.to_string(),
            "user_id": claims.sub_id
        }))))
    }

    /// GET /channels/:id/messages - Récupérer l'historique des messages d'un canal
    pub async fn get_message_history(
        State(handler): State<Arc<ChannelHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Récupérer les messages du canal depuis la DB
        // let messages = handler.message_uc.get_channel_messages(id).await?;
        // Vérifier que l'utilisateur a accès au canal
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "channel_id": id.to_string(),
            "user_id": claims.sub_id,
            "messages": []
        }))))
    }
}
