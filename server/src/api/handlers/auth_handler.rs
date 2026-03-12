use crate::application::dto::auth::{LoginRequest, SignupRequest};
use crate::application::use_cases::auth::{LoginUseCase, LogoutUseCase, SignupUseCase};
use crate::infrastructure::repositories::UserRepository;
use crate::infrastructure::security::JWTService;
use crate::utils::error::AppError;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

/// Handler contenant la logique métier pour l'authentification
#[derive(Clone)]
pub struct AuthHandler<R: UserRepository> {
    signup_uc: Arc<SignupUseCase<R>>,
    login_uc: Arc<LoginUseCase<R>>,
    logout_uc: Arc<LogoutUseCase<R>>,
    jwt_service: Arc<JWTService>,
}

impl<R: UserRepository> AuthHandler<R> {
    pub fn new(
        signup_uc: SignupUseCase<R>,
        login_uc: LoginUseCase<R>,
        logout_uc: LogoutUseCase<R>,
        jwt_service: JWTService,
    ) -> Self {
        Self {
            signup_uc: Arc::new(signup_uc),
            login_uc: Arc::new(login_uc),
            logout_uc: Arc::new(logout_uc),
            jwt_service: Arc::new(jwt_service),
        }
    }
}

/// - Créer un nouveau compte
#[utoipa::path(
    post,
    path = "/auth/signup",
    tag = "auth",
    request_body = SignupRequest,
    responses(
        (status = 201, description = "Compte créé avec succès", body = SignupResponse),
        (status = 400, description = "Erreur de validation"),
        (status = 409, description = "Email ou username déjà utilisé")
    )
)]
pub async fn signup<R: UserRepository>(
    State(handler): State<Arc<AuthHandler<R>>>,
    Json(req): Json<SignupRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = handler.signup_uc.execute(req).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

/// - Se connecter
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Connexion réussie", body = LoginResponse),
        (status = 401, description = "Identifiants invalides")
    )
)]
pub async fn login<R: UserRepository>(
    State(handler): State<Arc<AuthHandler<R>>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = handler.login_uc.execute(req).await?;
    Ok((StatusCode::OK, Json(response)))
}

/// - Se déconnecter
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "auth",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Déconnexion réussie", body = LogoutResponse),
        (status = 401, description = "Non authentifié")
    )
)]
pub async fn logout<R: UserRepository>(
    State(handler): State<Arc<AuthHandler<R>>>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            AppError::Unauthorized("Missing or invalid Authorization header".to_string())
        })?;

    let response = handler.logout_uc.execute(token.to_string()).await?;
    Ok((StatusCode::OK, Json(response)))
}

/// - Obtenir les infos de l'utilisateur connecté
#[utoipa::path(
    get,
    path = "/auth/me",
    tag = "auth",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Informations utilisateur"),
        (status = 401, description = "Non authentifié")
    )
)]
pub async fn get_me<R: UserRepository>(
    State(handler): State<Arc<AuthHandler<R>>>,
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

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "user_id": claims.sub_id,
            "exp": claims.exp,
            "iat": claims.iat
        })),
    ))
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::auth::{LoginRequest, SignupRequest};
    use crate::domain::entities::User;
    use crate::infrastructure::repositories::mocks::mock_user_repository::MockUserRepository;
    use crate::infrastructure::security::PasswordService;
    use crate::infrastructure::services::UserService;
    use uuid::Uuid;

    #[test]
    fn test_jwt_service_integration() {
        let jwt_service = JWTService::new("test_secret".to_string());
        let user_id = Uuid::new_v4();

        let token = jwt_service
            .create_token(user_id)
            .expect("Failed to create token");
        assert!(!token.is_empty());

        let claims = jwt_service
            .verify_token(&token)
            .expect("Failed to verify token");
        assert_eq!(claims.sub_id, user_id.to_string());
    }

    #[test]
    fn test_jwt_invalid_token() {
        let jwt_service = JWTService::new("test_secret".to_string());
        let result = jwt_service.verify_token("invalid_token");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_signup_success() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let signup_uc = SignupUseCase::new(user_service, jwt_service);

        let request = SignupRequest {
            username: "newuser".to_string(),
            email: "new@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = signup_uc.execute(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.user.username, "newuser");
        assert_eq!(response.user.email, "new@example.com");
        assert!(!response.token.is_empty());
    }

    #[tokio::test]
    async fn test_signup_duplicate_email() {
        let password_service = PasswordService::new();
        let existing_user = User {
            id: Uuid::new_v4(),
            username: "existing".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(existing_user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let signup_uc = SignupUseCase::new(user_service, jwt_service);

        let request = SignupRequest {
            username: "newuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = signup_uc.execute(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_signup_duplicate_username() {
        let password_service = PasswordService::new();
        let existing_user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "existing@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(existing_user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let signup_uc = SignupUseCase::new(user_service, jwt_service);

        let request = SignupRequest {
            username: "testuser".to_string(),
            email: "new@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = signup_uc.execute(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_success() {
        let password_service = PasswordService::new();
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password123").unwrap(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let login_uc = LoginUseCase::new(user_service, jwt_service);

        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = login_uc.execute(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.user.email, "test@example.com");
        assert_eq!(response.user.status, "ONLINE");
        assert!(!response.token.is_empty());
    }

    #[tokio::test]
    async fn test_login_invalid_credentials() {
        let password_service = PasswordService::new();
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password123").unwrap(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let login_uc = LoginUseCase::new(user_service, jwt_service);

        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };

        let result = login_uc.execute(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let login_uc = LoginUseCase::new(user_service, jwt_service);

        let request = LoginRequest {
            email: "notfound@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = login_uc.execute(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_logout_success() {
        let password_service = PasswordService::new();
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password123").unwrap(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

        let token = jwt_service.create_token(user_id).unwrap();
        let result = logout_uc.execute(token).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.message, "Vous êtes bien déconnecté");
    }

    #[tokio::test]
    async fn test_logout_already_offline() {
        let password_service = PasswordService::new();
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password123").unwrap(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

        let token = jwt_service.create_token(user_id).unwrap();
        let result = logout_uc.execute(token).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_logout_missing_token() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let signup_uc = SignupUseCase::new(user_service.clone(), jwt_service.clone());
        let login_uc = LoginUseCase::new(user_service.clone(), jwt_service.clone());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

        let handler = Arc::new(AuthHandler::new(signup_uc, login_uc, logout_uc, jwt_service));
        let headers = HeaderMap::new();

        let result = AuthHandler::logout(State(handler), headers).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_logout_invalid_token() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let signup_uc = SignupUseCase::new(user_service.clone(), jwt_service.clone());
        let login_uc = LoginUseCase::new(user_service.clone(), jwt_service.clone());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

        let handler = Arc::new(AuthHandler::new(signup_uc, login_uc, logout_uc, jwt_service));
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer invalid_token".parse().unwrap());

        let result = AuthHandler::logout(State(handler), headers).await;
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
            password_hash: password_service.hash("password123").unwrap(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let signup_uc = SignupUseCase::new(user_service.clone(), jwt_service.clone());
        let login_uc = LoginUseCase::new(user_service.clone(), jwt_service.clone());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

        let handler = Arc::new(AuthHandler::new(signup_uc, login_uc, logout_uc, jwt_service.clone()));
        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        let result = AuthHandler::get_me(State(handler), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_me_missing_token() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let signup_uc = SignupUseCase::new(user_service.clone(), jwt_service.clone());
        let login_uc = LoginUseCase::new(user_service.clone(), jwt_service.clone());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

        let handler = Arc::new(AuthHandler::new(signup_uc, login_uc, logout_uc, jwt_service));
        let headers = HeaderMap::new();

        let result = AuthHandler::get_me(State(handler), headers).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_me_invalid_token() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let signup_uc = SignupUseCase::new(user_service.clone(), jwt_service.clone());
        let login_uc = LoginUseCase::new(user_service.clone(), jwt_service.clone());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

        let handler = Arc::new(AuthHandler::new(signup_uc, login_uc, logout_uc, jwt_service));
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer invalid_token".parse().unwrap());

        let result = AuthHandler::get_me(State(handler), headers).await;
        assert!(result.is_err());
    }
}
