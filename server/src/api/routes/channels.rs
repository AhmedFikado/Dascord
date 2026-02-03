use axum::{routing::{get, put, delete}, Router};
use std::sync::Arc;
use crate::api::handlers::channel_handler::ChannelHandler;
use crate::infrastructure::repositories::{ChannelRepository, ServerRepository};

pub fn channel_routes<CR: ChannelRepository + 'static, SR: ServerRepository + 'static>(
    handler: Arc<ChannelHandler<CR, SR>>
) -> Router {
    Router::new()
        .route("/:id", get(ChannelHandler::<CR, SR>::get_channel_info))
        .route("/:id", put(ChannelHandler::<CR, SR>::update_channel))
        .route("/:id", delete(ChannelHandler::<CR, SR>::delete_channel))
        .with_state(handler)
}