use crate::application::use_cases::user::*;
use crate::infrastructure::repositories::UserRepository;
use crate::infrastructure::security::JWTService;
use crate::infrastructure::services::UserService;
use crate::infrastructure::websocket::ConnectionManager;
use crate::utils::error::AppError;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserHandler<R: UserRepository> {
    jwt_service: Arc<JWTService>,
    get_user_info_uc: Arc<GetUserInfoUseCase<R>>,
    update_status_uc: Arc<UpdateUserStatusUseCase<R>>,
}

impl<R: UserRepository> UserHandler<R> {
    pub fn new(user_service: UserService<R>, jwt_service: JWTService) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service.clone()),
            get_user_info_uc: Arc::new(GetUserInfoUseCase::new(user_service.clone())),
            update_status_uc: Arc::new(UpdateUserStatusUseCase::new(user_service)),
        }
    }

    pub fn with_ws_manager(mut self, ws_manager: Arc<ConnectionManager>) -> Self {
        self.update_status_uc = Arc::new(
            Arc::try_unwrap(self.update_status_uc).unwrap_or_else(|arc| (*arc).clone())
                .with_ws_manager(ws_manager)
        );
        self
    }

    pub async fn get_me(
        State(handler): State<Arc<UserHandler<R>>>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| {
                AppError::Unauthorized("Missing or invalid Authorization header".to_string())
            })?;

        let claims = handler.jwt_service.verify_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

        let user = handler.get_user_info_uc.execute(user_id).await?;
        Ok((StatusCode::OK, Json(user)))
    }

    pub async fn update_status(
        State(handler): State<Arc<UserHandler<R>>>,
        headers: HeaderMap,
        Json(payload): Json<serde_json::Value>,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| {
                AppError::Unauthorized("Missing or invalid Authorization header".to_string())
            })?;

        let claims = handler.jwt_service.verify_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

        let status = payload
            .get("status")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::ValidationError("Missing status field".to_string()))?
            .to_string();

        let user = handler.update_status_uc.execute(user_id, status).await?;
        Ok((StatusCode::OK, Json(user)))
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::User;
    use crate::infrastructure::repositories::mocks::mock_user_repository::MockUserRepository;
    use crate::infrastructure::security::PasswordService;

    #[tokio::test]
    async fn test_get_me_missing_token() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(UserHandler::new(user_service, jwt_service));

        let headers = HeaderMap::new();
        let result = UserHandler::get_me(State(handler), headers).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_me_invalid_token() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(UserHandler::new(user_service, jwt_service));

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer invalid_token".parse().unwrap());
        let result = UserHandler::get_me(State(handler), headers).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_me_success() {
        let password_service = PasswordService::new();
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(UserHandler::new(user_service, jwt_service.clone()));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result = UserHandler::get_me(State(handler), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_status_missing_token() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(UserHandler::new(user_service, jwt_service));

        let headers = HeaderMap::new();
        let payload = serde_json::json!({"status": "ONLINE"});
        let result = UserHandler::update_status(State(handler), headers, Json(payload)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_status_invalid_token() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(UserHandler::new(user_service, jwt_service));

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer invalid_token".parse().unwrap());
        let payload = serde_json::json!({"status": "ONLINE"});
        let result = UserHandler::update_status(State(handler), headers, Json(payload)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_status_missing_status_field() {
        let password_service = PasswordService::new();
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(UserHandler::new(user_service, jwt_service.clone()));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({});
        let result = UserHandler::update_status(State(handler), headers, Json(payload)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_status_success() {
        let password_service = PasswordService::new();
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(UserHandler::new(user_service, jwt_service.clone()));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"status": "ONLINE"});
        let result = UserHandler::update_status(State(handler), headers, Json(payload)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_status_invalid_status() {
        let password_service = PasswordService::new();
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(UserHandler::new(user_service, jwt_service.clone()));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"status": "INVALID"});
        let result = UserHandler::update_status(State(handler), headers, Json(payload)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_status_with_ws_manager() {
        let password_service = PasswordService::new();
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let ws_manager = Arc::new(ConnectionManager::new());
        let handler = Arc::new(UserHandler::new(user_service, jwt_service.clone()).with_ws_manager(ws_manager));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"status": "ONLINE"});
        let result = UserHandler::update_status(State(handler), headers, Json(payload)).await;

        assert!(result.is_ok());
    }
}
