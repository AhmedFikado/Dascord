use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::use_cases::message::*;
use crate::infrastructure::repositories::{MessageRepository, ChannelRepository, ServerRepository, UserRepository};
use crate::infrastructure::security::JWTService;
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct MessageHandler<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository, UR: UserRepository> {
    jwt_service: Arc<JWTService>,
    send_message_uc: Arc<SendMessageUseCase<MR, CR, SR>>,
    get_history_uc: Arc<GetMessageHistoryUseCase<MR, CR, SR>>,
    delete_message_uc: Arc<DeleteMessageUseCase<MR>>,
    user_repo: Arc<UR>,
}

impl<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository, UR: UserRepository> MessageHandler<MR, CR, SR, UR> {
    pub fn new(jwt_service: JWTService, message_repo: MR, channel_repo: CR, server_repo: SR, user_repo: UR) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service),
            send_message_uc: Arc::new(SendMessageUseCase::new(message_repo.clone(), channel_repo.clone(), server_repo.clone())),
            get_history_uc: Arc::new(GetMessageHistoryUseCase::new(message_repo.clone(), channel_repo, server_repo)),
            delete_message_uc: Arc::new(DeleteMessageUseCase::new(message_repo)),
            user_repo: Arc::new(user_repo),
        }
    }

    pub async fn send_message(
        State(handler): State<Arc<Self>>,
        Path(channel_id): Path<Uuid>,
        headers: HeaderMap,
        Json(payload): Json<serde_json::Value>,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
        
        let content = payload.get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::ValidationError("Missing content field".to_string()))?
            .to_string();
        
        let user = handler.user_repo.find_by_id(user_id).await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
        
        let message = handler.send_message_uc.execute(channel_id, user_id, user.username, content).await?;
        Ok((StatusCode::CREATED, Json(message)))
    }

    pub async fn get_message_history(
        State(handler): State<Arc<Self>>,
        Path(channel_id): Path<Uuid>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
        
        let messages = handler.get_history_uc.execute(channel_id, user_id).await?;
        Ok((StatusCode::OK, Json(messages)))
    }

    pub async fn delete_message(
        State(handler): State<Arc<Self>>,
        Path(id): Path<String>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
        
        handler.delete_message_uc.execute(id, user_id).await?;
        Ok((StatusCode::OK, Json(serde_json::json!({"message": "Message deleted"}))))
    }
}
