use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::application::use_cases::auth::{LoginUseCase, LogoutUseCase, SignupUseCase};
use crate::application::dto::auth::{LoginRequest, SignupRequest};
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct AuthHandler {
    signup_uc: Arc<SignupUseCase>,
    login_uc: Arc<LoginUseCase>,
    logout_uc: Arc<LogoutUseCase>,
}

impl AuthHandler {
    pub fn new(signup_uc: SignupUseCase, login_uc: LoginUseCase, logout_uc: LogoutUseCase) -> Self {
        Self { 
            signup_uc: Arc::new(signup_uc), 
            login_uc: Arc::new(login_uc),
            logout_uc: Arc::new(logout_uc),
        }
    }

    pub async fn signup(
        State(handler): State<Arc<AuthHandler>>,
        Json(req): Json<SignupRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = handler.signup_uc.execute(req).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }

    pub async fn login(
        State(handler): State<Arc<AuthHandler>>,
        Json(req): Json<LoginRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let response = handler.login_uc.execute(req).await?;
        Ok((StatusCode::OK, Json(response)))
    }

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
}