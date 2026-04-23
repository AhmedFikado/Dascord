use crate::application::controller::read_status_controller::{
    get_unread_channels, get_unread_private_channels, mark_channel_read, ReadStatusHandler,
};
use crate::infrastructure::repositories::channel::ChannelRepository;
use crate::infrastructure::repositories::read_status_repository::ReadStatusRepository;
use crate::infrastructure::repositories::ServerRepository;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn read_status_routes<
    RSR: ReadStatusRepository + 'static,
    CR: ChannelRepository + 'static,
    SR: ServerRepository + 'static,
>(
    handler: Arc<ReadStatusHandler<RSR, CR, SR>>,
) -> Router {
    Router::new()
        .route("/channels/:id/read", post(mark_channel_read::<RSR, CR, SR>))
        .route("/channels/private/unread", get(get_unread_private_channels::<RSR, CR, SR>))
        .route("/servers/:id/unread", get(get_unread_channels::<RSR, CR, SR>))
        .with_state(handler)
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::mocks::{
        mock_channel_repository::MockChannelRepository,
        mock_read_status_repository::MockReadStatusRepository,
        mock_server_repository::MockServerRepository,
    };
    use crate::infrastructure::security::JWTService;

    #[test]
    fn test_read_status_routes_creation() {
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_read_repo = MockReadStatusRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(ReadStatusHandler::new(
            jwt_service,
            mock_read_repo,
            mock_channel_repo,
            mock_server_repo,
        ));
        let _router = read_status_routes::<
            MockReadStatusRepository,
            MockChannelRepository,
            MockServerRepository,
        >(handler);
        assert!(true);
    }
}
