use axum::{routing::{get, post, put, delete}, Router};
use std::sync::Arc;
use crate::api::handlers::channel_handler::ChannelHandler;

/// Configure les routes des canaux
pub fn channel_routes(handler: Arc<ChannelHandler>) -> Router {
    Router::new()
        .route("/:id", get(ChannelHandler::get_channel_info))
        .route("/:id", put(ChannelHandler::update_channel))
        .route("/:id", delete(ChannelHandler::delete_channel))
        .route("/:id/messages", post(ChannelHandler::send_message))
        .route("/:id/messages", get(ChannelHandler::get_message_history))
        .with_state(handler)
}