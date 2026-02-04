use crate::api::router;
use crate::application::use_cases::auth::{LoginUseCase, LogoutUseCase, SignupUseCase};
use crate::infrastructure::security::jwt::JWTService;
use crate::infrastructure::repositories::PostgresUserRepository;
use crate::infrastructure::services::UserService;
use sqlx::PgPool;
use mongodb::Client as MongoClient;

pub async fn start_app(pg_pool: PgPool, mongo_client: MongoClient) {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev_secret_key".to_string());
    let jwt_service = JWTService::new(jwt_secret);

    let user_repo = PostgresUserRepository::new(pg_pool.clone());
    let user_service = UserService::new(user_repo);

    let signup_uc = SignupUseCase::new(user_service.clone(), jwt_service.clone());
    let login_uc = LoginUseCase::new(user_service.clone(), jwt_service.clone());
    let logout_uc = LogoutUseCase::new(user_service.clone(), jwt_service.clone());

    let app = router::create_router(signup_uc, login_uc, logout_uc, jwt_service, user_service, pg_pool, mongo_client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("🚀 Server starting on http://0.0.0.0:8080");

    axum::serve(listener, app).await.unwrap();
}
