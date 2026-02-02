use axum::{routing::{get, post, put, delete}, Router};
use std::sync::Arc;
use crate::api::handlers::server_handler::ServerHandler;

/// Configure les routes des serveurs
pub fn server_routes(handler: Arc<ServerHandler>) -> Router {
    Router::new()
        .route("/", post(ServerHandler::create_server))
        .route("/", get(ServerHandler::get_user_servers))
        .route("/:id", get(ServerHandler::get_server_info))
        .route("/:id", put(ServerHandler::update_server))
        .route("/:id", delete(ServerHandler::delete_server))
        .route("/:id/join", post(ServerHandler::join_server))
        .route("/:id/leave", delete(ServerHandler::leave_server))
        .route("/:id/members", get(ServerHandler::list_members))
        .route("/:id/members/:userId", put(ServerHandler::update_member_role))
        .route("/:id/channels", get(ServerHandler::get_channels))
        .route("/:id/channels", post(ServerHandler::create_channel))
        .with_state(handler)
}