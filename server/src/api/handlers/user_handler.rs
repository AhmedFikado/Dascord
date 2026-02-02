use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::infrastructure::security::JWTService;
use crate::mocks::MockUserService;
use crate::mocks::mock_user_repo::UserRepository;
use crate::utils::error::AppError;
use uuid::Uuid;

/// Handler pour les opérations utilisateur
#[derive(Clone)]
pub struct UserHandler {
    user_service: Arc<MockUserService>,
    jwt_service: Arc<JWTService>,
}

impl UserHandler {
    pub fn new(user_service: MockUserService, jwt_service: JWTService) -> Self {
        Self {
            user_service: Arc::new(user_service),
            jwt_service: Arc::new(jwt_service),
        }
    }

    /// GET /users/me - Obtenir les informations de l'utilisateur connecté
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
        
        // Récupérer l'utilisateur complet
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;
        
        let user = handler.user_service.repo()
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "id": user.id.to_string(),
            "username": user.username,
            "email": user.email,
            "status": user.status,
            "created_at": user.created_at,
        }))))
    }
}
