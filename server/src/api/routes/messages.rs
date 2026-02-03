use axum::{routing::{get, post, delete}, Router};
use std::sync::Arc;
use crate::api::handlers::message_handler::MessageHandler;
use crate::infrastructure::repositories::{MessageRepository, ChannelRepository, ServerRepository};

pub fn message_routes<MR: MessageRepository + 'static, CR: ChannelRepository + 'static, SR: ServerRepository + 'static>(
    handler: Arc<MessageHandler<MR, CR, SR>>
) -> Router {
    Router::new()
        .route("/channels/:channel_id/messages", post(MessageHandler::<MR, CR, SR>::send_message))
        .route("/channels/:channel_id/messages", get(MessageHandler::<MR, CR, SR>::get_message_history))
        .route("/:id", delete(MessageHandler::<MR, CR, SR>::delete_message))
        .with_state(handler)
}