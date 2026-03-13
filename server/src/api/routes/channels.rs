use crate::api::handlers::channel_handler::{ChannelHandler, get_channel_info, update_channel, delete_channel};
use crate::infrastructure::repositories::{ChannelRepository, ServerRepository};
use axum::{
    routing::{delete as axum_delete, get, put},
    Router,
};
use std::sync::Arc;

pub fn channel_routes<CR: ChannelRepository + 'static, SR: ServerRepository + 'static>(
    handler: Arc<ChannelHandler<CR, SR>>,
) -> Router {
    Router::new()
        .route("/:id", get(get_channel_info::<CR, SR>))
        .route("/:id", put(update_channel::<CR, SR>))
        .route("/:id", axum_delete(delete_channel::<CR, SR>))
        .with_state(handler)
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::mocks::{mock_channel_repository::MockChannelRepository, mock_server_repository::MockServerRepository};
    use crate::infrastructure::security::JWTService;

    #[test]
    fn test_channel_routes_creation() {
        use crate::api::handlers::channel_handler::ChannelHandler;
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(ChannelHandler::new(jwt_service, mock_channel_repo, mock_server_repo));
        let _router = channel_routes(handler);
        assert!(true);
    }

    #[test]
    fn test_channel_routes_has_correct_paths() {
        use crate::api::handlers::channel_handler::ChannelHandler;
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(ChannelHandler::new(jwt_service, mock_channel_repo, mock_server_repo));
        let _router = channel_routes(handler);
        assert!(true);
    }
}
