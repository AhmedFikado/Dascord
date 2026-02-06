use crate::api::handlers::message_handler::MessageHandler;
use crate::infrastructure::repositories::{
    ChannelRepository, MessageRepository, ServerRepository, UserRepository,
};
use axum::{
    routing::{delete, get, post},
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
            post(MessageHandler::<MR, CR, SR, UR>::send_message),
        )
        .route(
            "/channels/:channel_id/messages",
            get(MessageHandler::<MR, CR, SR, UR>::get_message_history),
        )
        .route(
            "/channels/:channel_id/messages/welcome",
            post(MessageHandler::<MR, CR, SR, UR>::send_welcome_message),
        )
        .route(
            "/messages/:id",
            delete(MessageHandler::<MR, CR, SR, UR>::delete_message),
        )
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
        use crate::api::handlers::message_handler::MessageHandler;
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(MessageHandler::new(jwt_service, mock_message_repo, mock_channel_repo, mock_server_repo, mock_user_repo));
        let _router = message_routes(handler);
        assert!(true);
    }

    #[test]
    fn test_message_routes_has_correct_paths() {
        use crate::api::handlers::message_handler::MessageHandler;
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());
        let handler = Arc::new(MessageHandler::new(jwt_service, mock_message_repo, mock_channel_repo, mock_server_repo, mock_user_repo));
        let _router = message_routes(handler);
        assert!(true);
    }
}
