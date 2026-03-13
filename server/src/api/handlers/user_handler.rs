use crate::application::use_cases::user::*;
use crate::domain::entities::User;
use crate::infrastructure::repositories::{ServerRepository, UserRepository};
use crate::infrastructure::security::JWTService;
use crate::infrastructure::services::UserService;
use crate::infrastructure::websocket::{ConnectionManager, ServerMessage};
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
pub struct UserHandler<R: UserRepository, SR: ServerRepository> {
    jwt_service: Arc<JWTService>,
    get_user_info_uc: Arc<GetUserInfoUseCase<R>>,
    update_status_uc: Arc<UpdateUserStatusUseCase<R>>,
    update_user_uc: Arc<UpdateUserInfoUseCase<R>>,
    ws_manager: Option<Arc<ConnectionManager>>,
    server_repo: Option<Arc<SR>>,
}

impl<R: UserRepository, SR: ServerRepository> UserHandler<R, SR> {
    pub fn new(user_service: UserService<R>, jwt_service: JWTService) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service.clone()),
            get_user_info_uc: Arc::new(GetUserInfoUseCase::new(user_service.clone())),
            update_status_uc: Arc::new(UpdateUserStatusUseCase::new(user_service.clone())),
            update_user_uc: Arc::new(UpdateUserInfoUseCase::new(user_service)),
            ws_manager: None,
            server_repo: None,
        }
    }

    pub fn with_ws_manager(mut self, ws_manager: Arc<ConnectionManager>) -> Self {
        self.update_status_uc = Arc::new(
            Arc::try_unwrap(self.update_status_uc)
                .unwrap_or_else(|arc| (*arc).clone())
                .with_ws_manager(ws_manager.clone()),
        );
        self.ws_manager = Some(ws_manager);
        self
    }

    pub fn with_server_repo(mut self, server_repo: SR) -> Self {
        self.server_repo = Some(Arc::new(server_repo));
        self
    }
}

/// - Obtenir les informations de l'utilisateur connecté
#[utoipa::path(
    get,
    path = "/users/me",
    tag = "users",
    responses(
        (status = 200, description = "Informations utilisateur", body = UserResponse),
        (status = 401, description = "Non autorisé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_me<R: UserRepository, SR: ServerRepository>(
    State(handler): State<Arc<UserHandler<R, SR>>>,
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

/// - Mettre à jour le statut de l'utilisateur connecté
#[utoipa::path(
    put,
    path = "/users/me/status",
    tag = "users",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Statut mis à jour", body = UserResponse),
        (status = 401, description = "Non autorisé"),
        (status = 400, description = "Statut invalide")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_status<R: UserRepository, SR: ServerRepository>(
    State(handler): State<Arc<UserHandler<R, SR>>>,
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

/// - Mettre à jour les informations de l'utilisateur
#[utoipa::path(
    put,
    path = "/users/update_user",
    tag = "users",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Utilisateur mis à jour", body = UserResponse),
        (status = 401, description = "Non autorisé"),
        (status = 400, description = "Données invalides")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_user<R: UserRepository, SR: ServerRepository>(
    State(handler): State<Arc<UserHandler<R, SR>>>,
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

    // Récupérer l'utilisateur actuel pour préserver le language s'il n'est pas fourni
    let current_user = handler.get_user_info_uc.execute(user_id).await?;

    let username = payload
        .get("username")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ValidationError("Missing username field".to_string()))?
        .to_string();

    let email = payload
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ValidationError("Missing email field".to_string()))?
        .to_string();

    let language = payload
        .get("language")
        .and_then(|v| v.as_str())
        .unwrap_or(&current_user.language)
        .to_string();

    let user = User {
        id: user_id,
        username,
        email,
        password_hash: String::new(),
        language,
        status: String::new(),
        created_at: chrono::Utc::now(),
    };

    let updated_user = handler.update_user_uc.execute(user).await?;

    if let (Some(ws_manager), Some(server_repo)) = (&handler.ws_manager, &handler.server_repo) {
        match server_repo.find_by_user(user_id).await {
            Ok(servers) => {
                for server in servers {
                    ws_manager
                        .broadcast_to_all(ServerMessage::ServerMemberUpdated {
                            server_id: server.id.to_string(),
                            user_id: user_id.to_string(),
                            username: updated_user.username.clone(),
                        })
                        .await;
                }
            }
            Err(e) => {
                tracing::error!("Failed to find servers for user {}: {:?}", user_id, e);
            }
        }
    }

    Ok((StatusCode::OK, Json(updated_user)))
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::User;
    use crate::infrastructure::repositories::mocks::mock_server_repository::MockServerRepository;
    use crate::infrastructure::repositories::mocks::mock_user_repository::MockUserRepository;
    use crate::infrastructure::security::PasswordService;

    type TestHandler = UserHandler<MockUserRepository, MockServerRepository>;

    #[tokio::test]
    async fn test_get_me_missing_token() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(TestHandler::new(user_service, jwt_service));

        let headers = HeaderMap::new();
        let result = UserHandler::get_me(State(handler), headers).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_me_invalid_token() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(TestHandler::new(user_service, jwt_service));

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
            language: "en".to_string(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(TestHandler::new(user_service, jwt_service.clone()));

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
        let handler = Arc::new(TestHandler::new(user_service, jwt_service));

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
        let handler = Arc::new(TestHandler::new(user_service, jwt_service));

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
            language: "en".to_string(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(TestHandler::new(user_service, jwt_service.clone()));

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
            language: "en".to_string(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(TestHandler::new(user_service, jwt_service.clone()));

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
            language: "en".to_string(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(TestHandler::new(user_service, jwt_service.clone()));

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
            language: "en".to_string(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let ws_manager = Arc::new(ConnectionManager::new());
        let handler = Arc::new(
            TestHandler::new(user_service, jwt_service.clone()).with_ws_manager(ws_manager),
        );

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
    async fn test_update_user_missing_token() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(TestHandler::new(user_service, jwt_service));

        let headers = HeaderMap::new();
        let payload = serde_json::json!({"username": "newname", "email": "new@example.com"});
        let result = UserHandler::update_user(State(handler), headers, Json(payload)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_invalid_token() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(TestHandler::new(user_service, jwt_service));

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer invalid_token".parse().unwrap());
        let payload = serde_json::json!({"username": "newname", "email": "new@example.com"});
        let result = UserHandler::update_user(State(handler), headers, Json(payload)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_missing_username_field() {
        let password_service = PasswordService::new();
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            language: "en".to_string(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(TestHandler::new(user_service, jwt_service.clone()));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"email": "new@example.com"});
        let result = UserHandler::update_user(State(handler), headers, Json(payload)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_missing_email_field() {
        let password_service = PasswordService::new();
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            language: "en".to_string(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(TestHandler::new(user_service, jwt_service.clone()));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"username": "newname"});
        let result = UserHandler::update_user(State(handler), headers, Json(payload)).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_success() {
        let password_service = PasswordService::new();
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: "oldname".to_string(),
            email: "old@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            language: "en".to_string(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(TestHandler::new(user_service, jwt_service.clone()));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"username": "newname", "email": "new@example.com"});
        let result = UserHandler::update_user(State(handler), headers, Json(payload)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_user_not_found() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(TestHandler::new(user_service, jwt_service.clone()));

        let token = jwt_service.create_token(Uuid::new_v4()).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"username": "ghost", "email": "ghost@example.com"});
        let result = UserHandler::update_user(State(handler), headers, Json(payload)).await;

        assert!(result.is_err());
    }
}
