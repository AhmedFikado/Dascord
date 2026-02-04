use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::use_cases::channel::*;
use crate::infrastructure::repositories::{ChannelRepository, ServerRepository};
use crate::infrastructure::security::JWTService;
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct ChannelHandler<CR: ChannelRepository, SR: ServerRepository> {
    jwt_service: Arc<JWTService>,
    get_channel_info_uc: Arc<GetChannelInfoUseCase<CR, SR>>,
    update_channel_uc: Arc<UpdateChannelUseCase<CR, SR>>,
    delete_channel_uc: Arc<DeleteChannelUseCase<CR, SR>>,
}

impl<CR: ChannelRepository, SR: ServerRepository> ChannelHandler<CR, SR> {
    pub fn new(jwt_service: JWTService, channel_repo: CR, server_repo: SR) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service),
            get_channel_info_uc: Arc::new(GetChannelInfoUseCase::new(channel_repo.clone(), server_repo.clone())),
            update_channel_uc: Arc::new(UpdateChannelUseCase::new(channel_repo.clone(), server_repo.clone())),
            delete_channel_uc: Arc::new(DeleteChannelUseCase::new(channel_repo, server_repo)),
        }
    }

    pub async fn get_channel_info(
        State(handler): State<Arc<Self>>,
        Path(id): Path<Uuid>,
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
        
        let channel = handler.get_channel_info_uc.execute(id, user_id).await?;
        Ok((StatusCode::OK, Json(channel)))
    }

    pub async fn update_channel(
        State(handler): State<Arc<Self>>,
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
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
        
        let name = payload.get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::ValidationError("Missing name field".to_string()))?
            .to_string();
        
        let channel = handler.update_channel_uc.execute(id, user_id, name).await?;
        Ok((StatusCode::OK, Json(channel)))
    }

    pub async fn delete_channel(
        State(handler): State<Arc<Self>>,
        Path(id): Path<Uuid>,
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
        
        handler.delete_channel_uc.execute(id, user_id).await?;
        Ok((StatusCode::OK, Json(serde_json::json!({"message": "Channel deleted"}))))
    }
}
