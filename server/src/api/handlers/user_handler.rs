use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::application::use_cases::user::*;
use crate::infrastructure::security::JWTService;
use crate::infrastructure::services::UserService;
use crate::utils::error::AppError;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserHandler {
    jwt_service: Arc<JWTService>,
    get_user_info_uc: Arc<GetUserInfoUseCase>,
}

impl UserHandler {
    pub fn new(user_service: UserService, jwt_service: JWTService) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service.clone()),
            get_user_info_uc: Arc::new(GetUserInfoUseCase::new(user_service)),
        }
    }

    pub async fn get_me(
        State(handler): State<Arc<UserHandler>>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;
        
        let user = handler.get_user_info_uc.execute(user_id).await?;
        Ok((StatusCode::OK, Json(user)))
    }
}
