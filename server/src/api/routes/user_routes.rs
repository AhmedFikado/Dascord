use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use crate::api::handlers::UserHandler;

/// Routes pour les utilisateurs
pub fn user_routes(handler: Arc<UserHandler>) -> Router {
    Router::new()
        .route("/me", get({
            let handler = handler.clone();
            move |headers| UserHandler::get_me(axum::extract::State(handler.clone()), headers)
        }))
}
