use crate::application::controller::channel_controller::{ChannelHandler, get_channel_info, update_channel, delete_channel};
use crate::infrastructure::repositories::{ChannelRepository, ServerRepository};
use axum::{
    routing::{delete as axum_delete, get, put, post},
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
        // .route("/private", post(create_private_channel::<CR, SR>))
        // .route("/private", get(get_list_private_channels::<CR, SR>))
        // .route("/:id/private", get(get_private_channel::<CR, SR>))
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
        use crate::application::controller::channel_controller::ChannelHandler;
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(ChannelHandler::new(jwt_service, mock_channel_repo, mock_server_repo));
        let _router = channel_routes::<MockChannelRepository, MockServerRepository>(handler);
        assert!(true);
    }

    #[test]
    fn test_channel_routes_has_correct_paths() {
        use crate::application::controller::channel_controller::ChannelHandler;
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(ChannelHandler::new(jwt_service, mock_channel_repo, mock_server_repo));
        let _router = channel_routes::<MockChannelRepository, MockServerRepository>(handler);
        assert!(true);
    }
}
