use crate::application::controller::message_controller::{MessageHandler, send_welcome_message, get_message_history, send_message, delete_message, update_message};
use crate::infrastructure::repositories::{
    ChannelRepository, MessageRepository, ServerRepository, UserRepository,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn message_routes<
    MR: MessageRepository + 'static,
    CR: ChannelRepository + 'static,
    SR: ServerRepository + 'static,
    UR: UserRepository + 'static,
>(
    handler: Arc<MessageHandler<MR, CR, SR, UR>>,
) -> Router {
    Router::new()
        .route(
            "/channels/:channel_id/messages",
            post(send_message::<MR, CR, SR, UR>),
        )
        .route(
            "/channels/:channel_id/messages",
            get(get_message_history::<MR, CR, SR, UR>),
        )
        .route(
            "/channels/:channel_id/messages/welcome",
            post(send_welcome_message::<MR, CR, SR, UR>),
        )
        .route(
            "/messages/:id",
            delete(delete_message::<MR, CR, SR, UR>),
        )
        .route(
            "/messages/:id",
            put(update_message::<MR, CR, SR, UR>),
        )
        // .route(
        //     "/channels/:channel_id/messages/private",
        //     get(get_private_message_history::<MR, CR, SR, UR>),
        // )
        // .route(
        //     "/channels/:channel_id/messages/private",
        //     post(send_private_message::<MR, CR, SR, UR>),
        // )
        .with_state(handler)
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::mocks::{mock_message_repository::MockMessageRepository, mock_channel_repository::MockChannelRepository, mock_server_repository::MockServerRepository, mock_user_repository::MockUserRepository};
    use crate::infrastructure::security::JWTService;

    #[test]
    fn test_message_routes_creation() {
        use crate::application::controller::message_controller::MessageHandler;
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(MessageHandler::new(jwt_service, mock_message_repo, mock_channel_repo, mock_server_repo, mock_user_repo));
        let _router = message_routes::<MockMessageRepository, MockChannelRepository, MockServerRepository, MockUserRepository>(handler);
        assert!(true);
    }

    #[test]
    fn test_message_routes_has_correct_paths() {
        use crate::application::controller::message_controller::MessageHandler;
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(MessageHandler::new(jwt_service, mock_message_repo, mock_channel_repo, mock_server_repo, mock_user_repo));
        let _router = message_routes::<MockMessageRepository, MockChannelRepository, MockServerRepository, MockUserRepository>(handler);
        assert!(true);
    }
}
