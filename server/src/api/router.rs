use axum::Router;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use crate::api::handlers::{AuthHandler, ChannelHandler, ServerHandler, MessageHandler, UserHandler};
use crate::api::routes::{auth_routes, channel_routes, server_routes, message_routes, user_routes};
use crate::application::use_cases::auth::{LoginUseCase, LogoutUseCase, SignupUseCase};
use crate::infrastructure::security::JWTService;
use crate::infrastructure::repositories::{PostgresServerRepository, PostgresChannelRepository, MongoMessageRepository};
use crate::infrastructure::services::UserService;
use sqlx::PgPool;
use mongodb::Client as MongoClient;

pub fn create_router(
    signup_uc: SignupUseCase,
    login_uc: LoginUseCase,
    logout_uc: LogoutUseCase,
    jwt_service: JWTService,
    user_service: UserService,
    pg_pool: PgPool,
    mongo_client: MongoClient,
) -> Router {
    let auth_handler = Arc::new(AuthHandler::new(
        signup_uc,
        login_uc,
        logout_uc,
        jwt_service.clone(),
    ));
    let user_handler = Arc::new(UserHandler::new(user_service, jwt_service.clone()));
    
    let server_repo = PostgresServerRepository::new(pg_pool.clone());
    let channel_repo = PostgresChannelRepository::new(pg_pool);
    let message_repo = MongoMessageRepository::new(mongo_client);
    
    let server_handler = Arc::new(ServerHandler::new(
        jwt_service.clone(),
        server_repo.clone(),
        channel_repo.clone(),
    ));
    let channel_handler = Arc::new(ChannelHandler::new(
        jwt_service.clone(),
        channel_repo.clone(),
        server_repo.clone(),
    ));
    let message_handler = Arc::new(MessageHandler::new(
        jwt_service.clone(),
        message_repo,
        channel_repo,
        server_repo,
    ));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .nest("/auth", auth_routes(auth_handler))
        .nest("/users", user_routes(user_handler))
        .nest("/servers", server_routes(server_handler))
        .nest("/channels", channel_routes(channel_handler))
        .nest("/messages", message_routes(message_handler))
        .layer(cors)
}
