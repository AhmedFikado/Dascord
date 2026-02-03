use axum::{routing::{get, post, put, delete}, Router};
use std::sync::Arc;
use crate::api::handlers::server_handler::ServerHandler;
use crate::infrastructure::repositories::{ServerRepository, ChannelRepository};

/// Configure les routes des serveurs
/// 
pub fn server_routes<SR: ServerRepository + 'static, CR: ChannelRepository + 'static>(
    handler: Arc<ServerHandler<SR, CR>>
) -> Router {
    Router::new()
        .route("/", post(ServerHandler::<SR, CR>::create_server))
        .route("/", get(ServerHandler::<SR, CR>::get_user_servers))
        .route("/:id", get(ServerHandler::<SR, CR>::get_server_info))
        .route("/:id", put(ServerHandler::<SR, CR>::update_server))
        .route("/:id", delete(ServerHandler::<SR, CR>::delete_server))
        .route("/:id/join", post(ServerHandler::<SR, CR>::join_server))
        .route("/:id/leave", delete(ServerHandler::<SR, CR>::leave_server))
        .route("/:id/members", get(ServerHandler::<SR, CR>::list_members))
        .route("/:id/members/:userId", put(ServerHandler::<SR, CR>::update_member_role))
        .route("/:id/channels", get(ServerHandler::<SR, CR>::get_channels))
        .route("/:id/channels", post(ServerHandler::<SR, CR>::create_channel))
        .with_state(handler)
}