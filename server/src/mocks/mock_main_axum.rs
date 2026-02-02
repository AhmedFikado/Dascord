use crate::api::router;
use crate::application::use_cases::auth::{LoginUseCase, LogoutUseCase, SignupUseCase};
use crate::infrastructure::security::jwt::JWTService;
use crate::mocks::MockUserService;

pub async fn start_app() {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev_secret_key".to_string());
    let jwt_service = JWTService::new(jwt_secret);

    let user_service = MockUserService::new();

    let signup_uc = SignupUseCase::new(user_service.clone(), jwt_service.clone());
    let login_uc = LoginUseCase::new(user_service.clone(), jwt_service.clone());
    let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

    let app = router::create_router(signup_uc, login_uc, logout_uc, jwt_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("🚀 Server starting on http://0.0.0.0:8080");

    axum::serve(listener, app).await.unwrap();
}
