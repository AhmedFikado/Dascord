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

/// Handler contenant la logique métier pour les messages
#[derive(Clone)]
pub struct MessageHandler {
    jwt_service: Arc<JWTService>,
}

impl MessageHandler {
    pub fn new(jwt_service: JWTService) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service),
        }
    }

    /// DELETE /messages/:id - Supprimer un message
    pub async fn delete_message(
        State(handler): State<Arc<MessageHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Supprimer le message
        // let message = handler.message_uc.get_message_by_id(id).await?;
        // Vérifier que l'utilisateur est l'auteur du message ou admin
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "message": "Message deleted",
            "message_id": id.to_string(),
            "user_id": claims.sub_id
        }))))
    }
}
