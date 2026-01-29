use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use crate::application::use_cases::auth::{LoginUseCase, SignupUseCase};
use crate::application::dto::auth::{LoginRequest, SignupRequest};
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct AuthHandler {
    signup_uc: Arc<SignupUseCase>,
    login_uc: Arc<LoginUseCase>,
}

impl AuthHandler {
    pub fn new(signup_uc: SignupUseCase, login_uc: LoginUseCase) -> Self {
        Self { 
            signup_uc: Arc::new(signup_uc), 
            login_uc: Arc::new(login_uc) 
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
}