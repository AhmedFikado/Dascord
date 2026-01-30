use axum::{routing::delete, Router};
use std::sync::Arc;
use crate::api::handlers::message_handler::MessageHandler;

/// Configure les routes des messages
pub fn message_routes(handler: Arc<MessageHandler>) -> Router {
    Router::new()
        .route("/:id", delete(MessageHandler::delete_message))
        .with_state(handler)
}