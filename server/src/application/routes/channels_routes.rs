use crate::application::controller::channel::channel_controller::{ChannelHandler, get_channel_info, update_channel, delete_channel};
use crate::application::controller::channel::private_channel_controller::{
    create_private_channel, get_list_private_channels, get_private_channel, hide_private_channel, PrivateChannelController
};
use crate::infrastructure::repositories::{ServerRepository, UserRepository};
use crate::infrastructure::repositories::channel::{ChannelRepository, PrivateChannelRepository};
use axum::{
    routing::{delete as axum_delete, get, patch, put, post},
    Router,
};
use std::sync::Arc;

pub fn channel_routes<
CR: ChannelRepository + 'static, 
SR: ServerRepository + 'static, 
UR: UserRepository + 'static, 
PCR: PrivateChannelRepository + 'static
>(
    channel_controller: Arc<ChannelHandler<CR, SR>>,
    private_channel_controller: Arc<PrivateChannelController<PCR, UR>>,
) -> Router {
    Router::new()
        .route("/:id", get(get_channel_info::<CR, SR>))
        .route("/:id", put(update_channel::<CR, SR>))
        .route("/:id", axum_delete(delete_channel::<CR, SR>))
        .with_state(channel_controller)
        .route("/private", post(create_private_channel::<PCR, UR>))
        .route("/private", get(get_list_private_channels::<PCR, UR>))
        .route("/:id/private", get(get_private_channel::<PCR, UR>))
        .route("/private/:id/hide", patch(hide_private_channel::<PCR, UR>))
        .with_state(private_channel_controller)
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::mocks::{
        mock_channel_repository::MockChannelRepository, 
        mock_server_repository::MockServerRepository,
        mock_user_repository::MockUserRepository,
        mock_private_channel_repository::MockPrivateChannelRepository,
    };
    use crate::infrastructure::security::JWTService;

    #[test]
    fn test_channel_routes_creation() {
        use crate::application::controller::channel::channel_controller::ChannelHandler;
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let mock_private_channel_repo = MockPrivateChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(ChannelHandler::new(jwt_service.clone(), mock_channel_repo, mock_server_repo));
        let private_channel_controller = Arc::new(PrivateChannelController::new(jwt_service,mock_private_channel_repo, mock_user_repo));
        let _router = channel_routes::<MockChannelRepository, MockServerRepository, MockUserRepository, MockPrivateChannelRepository>(handler, private_channel_controller);
        assert!(true);
    }

    #[test]
    fn test_channel_routes_has_correct_paths() {
        use crate::application::controller::channel::channel_controller::ChannelHandler;
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let mock_private_channel_repo = MockPrivateChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(ChannelHandler::new(jwt_service.clone(), mock_channel_repo, mock_server_repo));
        let private_channel_controller = Arc::new(PrivateChannelController::new(jwt_service,mock_private_channel_repo, mock_user_repo));
        let _router = channel_routes::<MockChannelRepository, MockServerRepository, MockUserRepository, MockPrivateChannelRepository>(handler, private_channel_controller);
        assert!(true);
    }
}
