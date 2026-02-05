use axum::{routing::{get, post, put, delete}, Router};
use std::sync::Arc;
use crate::api::handlers::server_handler::ServerHandler;
use crate::infrastructure::repositories::{ServerRepository, ChannelRepository, UserRepository};

/// Configure les routes des serveurs
/// 
pub fn server_routes<SR: ServerRepository + 'static, CR: ChannelRepository + 'static, UR: UserRepository + 'static>(
    handler: Arc<ServerHandler<SR, CR, UR>>
) -> Router {
    Router::new()
        .route("/", post(ServerHandler::<SR, CR, UR>::create_server))
        .route("/", get(ServerHandler::<SR, CR, UR>::get_user_servers))
        .route("/:id", get(ServerHandler::<SR, CR, UR>::get_server_info))
        .route("/:id", put(ServerHandler::<SR, CR, UR>::update_server))
        .route("/:id", delete(ServerHandler::<SR, CR, UR>::delete_server))
        .route("/join", post(ServerHandler::<SR, CR, UR>::join_server))
        .route("/:id/leave", delete(ServerHandler::<SR, CR, UR>::leave_server))
        .route("/:id/members", get(ServerHandler::<SR, CR, UR>::list_members))
        .route("/:id/members/:userId", put(ServerHandler::<SR, CR, UR>::update_member_role))
        .route("/:id/channels", get(ServerHandler::<SR, CR, UR>::get_channels))
        .route("/:id/channels", post(ServerHandler::<SR, CR, UR>::create_channel))
        .with_state(handler)
}