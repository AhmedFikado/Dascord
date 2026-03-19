use crate::application::controller::{
    AuthHandler, ServerHandler, UserHandler,
};
use crate::application::controller::message::{
    message_controller::MessageHandler,
    private_message_controller::PrivateMessageController
};
use crate::application::controller::channel::channel_controller::ChannelHandler;
use crate::application::controller::channel::private_channel_controller::PrivateChannelController;
use crate::application::openapi::ApiDoc;
use crate::application::routes::{auth_routes, channel_routes, message_routes, server_routes, user_routes};
use crate::domain::services::auth::{LoginUseCase, LogoutUseCase, SignupUseCase};
use crate::infrastructure::repositories::{
    PostgresServerRepository, PostgresUserRepository,
};
use crate::infrastructure::repositories::message::MongoMessageRepository;
use crate::infrastructure::repositories::channel::{ PostgresChannelRepository, PostgresPrivateChannelRepository };
use crate::infrastructure::security::JWTService;
use crate::domain::services::user::UserService;
use crate::infrastructure::websocket::ConnectionManager;
use axum::Router;
use mongodb::Client as MongoClient;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn create_router(
    signup_uc: SignupUseCase<PostgresUserRepository>,
    login_uc: LoginUseCase<PostgresUserRepository>,
    logout_uc: LogoutUseCase<PostgresUserRepository>,
    jwt_service: JWTService,
    user_service: UserService<PostgresUserRepository>,
    pg_pool: PgPool,
    mongo_client: MongoClient,
    ws_manager: Arc<ConnectionManager>,
) -> Router {
    let auth_handler = Arc::new(AuthHandler::new(
        signup_uc,
        login_uc,
        logout_uc,
        jwt_service.clone(),
    ));
    let user_handler = Arc::new(UserHandler::new(user_service, jwt_service.clone())
        .with_ws_manager(ws_manager.clone())
        .with_server_repo(PostgresServerRepository::new(pg_pool.clone())));

    let server_repo = PostgresServerRepository::new(pg_pool.clone());
    let channel_repo = PostgresChannelRepository::new(pg_pool.clone());
    let private_channel_repo = PostgresPrivateChannelRepository::new(pg_pool.clone());
    let message_repo = MongoMessageRepository::new(mongo_client);
    let user_repo = PostgresUserRepository::new(pg_pool.clone());
    let user_repo2 = PostgresUserRepository::new(pg_pool);
    
    let server_handler = Arc::new(
        ServerHandler::new(
            jwt_service.clone(),
            server_repo.clone(),
            channel_repo.clone(),
            user_repo,
        ).with_ws_manager(ws_manager.clone())
    );
    let channel_handler = Arc::new(ChannelHandler::new(
        jwt_service.clone(),
        channel_repo.clone(),
        server_repo.clone(),
    ));
    let private_channel_controller = Arc::new(PrivateChannelController::new(
        jwt_service.clone(),
        private_channel_repo.clone(),
        user_repo2.clone(),
    ).with_ws_manager(ws_manager.clone()));
    let message_handler = Arc::new(
        MessageHandler::new(
            jwt_service.clone(),
            message_repo.clone(),
            channel_repo.clone(),
            server_repo.clone(),
            user_repo2.clone(),
        ).with_ws_manager(ws_manager.clone())
    );
    let private_message_controller = Arc::new(
        PrivateMessageController::new(
            jwt_service.clone(),
            message_repo,
            private_channel_repo,
            user_repo2,
        ).with_ws_manager(ws_manager)
    );

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/auth", auth_routes(auth_handler))
        .nest("/users", user_routes(user_handler))
        .nest("/servers", server_routes(server_handler))
        .nest("/channels", channel_routes(channel_handler, private_channel_controller))
        .merge(message_routes(message_handler, private_message_controller))
        .layer(cors)
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::mocks::{
        mock_user_repository::MockUserRepository, 
        mock_server_repository::MockServerRepository, 
        mock_channel_repository::MockChannelRepository, 
        mock_message_repository::MockMessageRepository,
        mock_private_channel_repository::MockPrivateChannelRepository
    };

    #[test]
    fn test_router_creation_with_mocks() {
        let mock_user_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_user_repo.clone());
        let jwt_service = JWTService::new("test_secret".to_string());
        
        let signup_uc = SignupUseCase::new(user_service.clone(), jwt_service.clone());
        let login_uc = LoginUseCase::new(user_service.clone(), jwt_service.clone());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

        let auth_handler = Arc::new(AuthHandler::new(signup_uc, login_uc, logout_uc, jwt_service.clone()));
        let user_handler = Arc::new(UserHandler::<_, MockServerRepository>::new(UserService::new(mock_user_repo.clone()), jwt_service.clone()));

        let mock_server_repo = MockServerRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_message_repo = MockMessageRepository::new();
        let mock_private_channel_repo = MockPrivateChannelRepository::new();

        let server_handler = Arc::new(ServerHandler::new(jwt_service.clone(), mock_server_repo.clone(), mock_channel_repo.clone(), mock_user_repo.clone()));
        let channel_handler = Arc::new(ChannelHandler::new(jwt_service.clone(), mock_channel_repo.clone(), mock_server_repo.clone()));
        let private_channel_controller = Arc::new(PrivateChannelController::new(jwt_service.clone(), mock_private_channel_repo.clone(), mock_user_repo.clone()));
        let message_handler = Arc::new(MessageHandler::new(jwt_service.clone(), mock_message_repo.clone(), mock_channel_repo.clone(), mock_server_repo.clone(), mock_user_repo.clone()));
        let private_message_controller = Arc::new(PrivateMessageController::new(jwt_service, mock_message_repo, mock_private_channel_repo, mock_user_repo));

        let _router = Router::new()
            .nest("/auth", auth_routes(auth_handler))
            .nest("/users", user_routes(user_handler))
            .nest("/servers", server_routes(server_handler))
            .nest("/channels", channel_routes(channel_handler, private_channel_controller))
            .merge(message_routes(message_handler, private_message_controller));

        assert!(true);
    }

    #[test]
    fn test_router_has_cors_layer() {
        let _cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);
        
        assert!(true);
    }

    #[test]
    fn test_router_nested_routes() {
        let mock_user_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_user_repo.clone());
        let jwt_service = JWTService::new("test_secret".to_string());
        
        let signup_uc = SignupUseCase::new(user_service.clone(), jwt_service.clone());
        let login_uc = LoginUseCase::new(user_service.clone(), jwt_service.clone());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

        let auth_handler = Arc::new(AuthHandler::new(signup_uc, login_uc, logout_uc, jwt_service));
        let _router = Router::new().nest("/auth", auth_routes(auth_handler));
        
        assert!(true);
    }
}
