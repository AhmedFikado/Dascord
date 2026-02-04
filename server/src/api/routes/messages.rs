use axum::{routing::{get, post, delete}, Router};
use std::sync::Arc;
use crate::api::handlers::message_handler::MessageHandler;
use crate::infrastructure::repositories::{MessageRepository, ChannelRepository, ServerRepository, UserRepository};

pub fn message_routes<MR: MessageRepository + 'static, CR: ChannelRepository + 'static, SR: ServerRepository + 'static, UR: UserRepository + 'static>(
    handler: Arc<MessageHandler<MR, CR, SR, UR>>
) -> Router {
    Router::new()
        .route("/channels/:channel_id/messages", post(MessageHandler::<MR, CR, SR, UR>::send_message))
        .route("/channels/:channel_id/messages", get(MessageHandler::<MR, CR, SR, UR>::get_message_history))
        .route("/messages/:id", delete(MessageHandler::<MR, CR, SR, UR>::delete_message))
        .with_state(handler)
}