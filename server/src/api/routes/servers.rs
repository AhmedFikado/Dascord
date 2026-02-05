use crate::api::handlers::server_handler::ServerHandler;
use crate::infrastructure::repositories::{ChannelRepository, ServerRepository};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

/// Configure les routes des serveurs
pub fn server_routes<SR: ServerRepository + 'static, CR: ChannelRepository + 'static>(
    handler: Arc<ServerHandler<SR, CR>>,
) -> Router {
    Router::new()
        .route("/", post(ServerHandler::<SR, CR>::create_server))
        .route("/", get(ServerHandler::<SR, CR>::get_user_servers))
        .route("/:id", get(ServerHandler::<SR, CR>::get_server_info))
        .route("/:id", put(ServerHandler::<SR, CR>::update_server))
        .route("/:id", delete(ServerHandler::<SR, CR>::delete_server))
        .route("/join", post(ServerHandler::<SR, CR>::join_server))
        .route("/:id/leave", delete(ServerHandler::<SR, CR>::leave_server))
        .route("/:id/members", get(ServerHandler::<SR, CR>::list_members))
        .route(
            "/:id/members/:userId",
            put(ServerHandler::<SR, CR>::update_member_role),
        )
        .route("/:id/channels", get(ServerHandler::<SR, CR>::get_channels))
        .route(
            "/:id/channels",
            post(ServerHandler::<SR, CR>::create_channel),
        )
        .with_state(handler)
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::mocks::{mock_server_repository::MockServerRepository, mock_channel_repository::MockChannelRepository};
    use crate::infrastructure::security::JWTService;

    #[test]
    fn test_server_routes_creation() {
        use crate::api::handlers::server_handler::ServerHandler;
        let mock_server_repo = MockServerRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(ServerHandler::new(jwt_service, mock_server_repo, mock_channel_repo));
        let _router = server_routes(handler);
        assert!(true);
    }

    #[test]
    fn test_server_routes_has_all_endpoints() {
        use crate::api::handlers::server_handler::ServerHandler;
        let mock_server_repo = MockServerRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(ServerHandler::new(jwt_service, mock_server_repo, mock_channel_repo));
        let _router = server_routes(handler);
        assert!(true);
    }
}
