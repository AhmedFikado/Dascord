use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::application::dto::auth::{LoginRequest, SignupRequest};
use crate::application::use_cases::auth::{LoginUseCase, LogoutUseCase, SignupUseCase};
use crate::infrastructure::security::JWTService;
use crate::utils::error::AppError;

/// Handler contenant la logique métier pour l'authentification
#[derive(Clone)]
pub struct AuthHandler {
    signup_uc: Arc<SignupUseCase>,
    login_uc: Arc<LoginUseCase>,
    logout_uc: Arc<LogoutUseCase>,
    jwt_service: Arc<JWTService>,
}

impl AuthHandler {
    pub fn new(
        signup_uc: SignupUseCase,
        login_uc: LoginUseCase,
        logout_uc: LogoutUseCase,
        jwt_service: JWTService,
    ) -> Self {
        Self {
            signup_uc: Arc::new(signup_uc),
            login_uc: Arc::new(login_uc),
            logout_uc: Arc::new(logout_uc),
            jwt_service: Arc::new(jwt_service),
        }
    }

    /// POST /auth/signup - Créer un nouveau compte
    pub async fn signup(
        State(handler): State<Arc<AuthHandler>>,
        Json(req): Json<SignupRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = handler.signup_uc.execute(req).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }

    /// POST /auth/login - Se connecter
    pub async fn login(
        State(handler): State<Arc<AuthHandler>>,
        Json(req): Json<LoginRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = handler.login_uc.execute(req).await?;
        Ok((StatusCode::OK, Json(response)))
    }

    /// POST /auth/logout - Se déconnecter
    pub async fn logout(
        State(handler): State<Arc<AuthHandler>>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let response = handler.logout_uc.execute(token.to_string()).await?;
        Ok((StatusCode::OK, Json(response)))
    }

    /// GET /auth/me - Obtenir les infos de l'utilisateur connecté
    pub async fn get_me(
        State(handler): State<Arc<AuthHandler>>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "user_id": claims.sub_id,
            "exp": claims.exp,
            "iat": claims.iat
        }))))
    }
}
