use axum::{routing::post, Router};
use std::sync::Arc;
use crate::api::handlers::auth_handler::AuthHandler;
use crate::application::use_cases::auth::{LoginUseCase, SignupUseCase};
use crate::infrastructure::security::jwt::JWTService;
use crate::mocks::MockUserService;

pub async fn start_app() {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev_secret_key".to_string());

    let user_service = MockUserService::new();
    let jwt_service = JWTService::new(jwt_secret);

    let signup_uc = SignupUseCase::new(user_service.clone(), jwt_service.clone());
    let login_uc = LoginUseCase::new(user_service, jwt_service);

    let auth_handler = Arc::new(AuthHandler::new(signup_uc, login_uc));

    let app = Router::new()
        .route("/auth/signup", post(AuthHandler::signup))
        .route("/auth/login", post(AuthHandler::login))
        .with_state(auth_handler);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("🚀 Server starting on http://127.0.0.1:8080");

    axum::serve(listener, app).await.unwrap();
}
