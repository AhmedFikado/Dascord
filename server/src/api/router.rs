use axum::Router;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use crate::api::handlers::{AuthHandler, ChannelHandler, ServerHandler, MessageHandler, UserHandler};
use crate::api::routes::{auth_routes, channel_routes, server_routes, message_routes, user_routes};
use crate::application::use_cases::auth::{LoginUseCase, LogoutUseCase, SignupUseCase};
use crate::infrastructure::security::JWTService;
use crate::mocks::MockUserService;

/// Crée le routeur principal de l'application avec toutes les routes configurées
pub fn create_router(
    signup_uc: SignupUseCase,
    login_uc: LoginUseCase,
    logout_uc: LogoutUseCase,
    jwt_service: JWTService,
    user_service: MockUserService,
) -> Router {
    // Créer le handler d'authentification
    let auth_handler = Arc::new(AuthHandler::new(
        signup_uc,
        login_uc,
        logout_uc,
        jwt_service.clone(),
    ));
    let user_handler = Arc::new(UserHandler::new(user_service, jwt_service.clone()));
    let server_handler = Arc::new(ServerHandler::new(jwt_service.clone()));
    let channel_handler = Arc::new(ChannelHandler::new(jwt_service.clone()));
    let message_handler = Arc::new(MessageHandler::new(jwt_service.clone()));

    // Configuration CORS pour autoriser les requêtes du frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Assembler toutes les routes
    Router::new()
        .nest("/auth", auth_routes(auth_handler))
        .nest("/users", user_routes(user_handler))
        .nest("/servers", server_routes(server_handler))
        .nest("/channels", channel_routes(channel_handler))
        .nest("/messages", message_routes(message_handler))
        .layer(cors)
}
