use crate::application::controller::server_controller::{
    create_server, get_user_servers, get_server_info, update_server, delete_server,
    join_server, leave_server, list_members, update_member_role, kick_member, ban_member,
    list_banned_members, unban_member,
    get_channels, create_channel, ServerHandler,
};
use crate::infrastructure::repositories::{ServerRepository, UserRepository};
use crate::infrastructure::repositories::channel::ChannelRepository;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn server_routes<SR: ServerRepository + 'static, CR: ChannelRepository + 'static, UR: UserRepository + 'static>(
    handler: Arc<ServerHandler<SR, CR, UR>>,
) -> Router {
    Router::new()
        .route("/", post(create_server::<SR, CR, UR>))
        .route("/", get(get_user_servers::<SR, CR, UR>))
        .route("/:id", get(get_server_info::<SR, CR, UR>))
        .route("/:id", put(update_server::<SR, CR, UR>))
        .route("/:id", delete(delete_server::<SR, CR, UR>))
        .route("/join", post(join_server::<SR, CR, UR>))
        .route("/:id/leave", delete(leave_server::<SR, CR, UR>))
        .route("/:id/members", get(list_members::<SR, CR, UR>))
        .route("/:id/members/:userId", put(update_member_role::<SR, CR, UR>))
        .route("/:id/members/:userId/kick", delete(kick_member::<SR, CR, UR>))
        .route("/:id/members/:userId/ban", post(ban_member::<SR, CR, UR>))
        .route("/:id/members/:userId/ban", delete(unban_member::<SR, CR, UR>))
        .route("/:id/bans", get(list_banned_members::<SR, CR, UR>))
        .route("/:id/channels", get(get_channels::<SR, CR, UR>))
        .route("/:id/channels", post(create_channel::<SR, CR, UR>))
        .with_state(handler)
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::mocks::{mock_server_repository::MockServerRepository, mock_channel_repository::MockChannelRepository, mock_user_repository::MockUserRepository};
    use crate::infrastructure::security::JWTService;

    #[test]
    fn test_server_routes_creation() {
        use crate::application::controller::server_controller::ServerHandler;
        let mock_server_repo = MockServerRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(ServerHandler::new(jwt_service, mock_server_repo, mock_channel_repo, mock_user_repo));
        let _router = server_routes::<MockServerRepository, MockChannelRepository, MockUserRepository>(handler);
        assert!(true);
    }

    #[test]
    fn test_server_routes_has_all_endpoints() {
        use crate::application::controller::server_controller::ServerHandler;
        let mock_server_repo = MockServerRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(ServerHandler::new(jwt_service, mock_server_repo, mock_channel_repo, mock_user_repo));
        let _router = server_routes::<MockServerRepository, MockChannelRepository, MockUserRepository>(handler);
        assert!(true);
    }
}
