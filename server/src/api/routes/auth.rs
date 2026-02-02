use axum::{routing::{get, post}, Router};
use std::sync::Arc;
use crate::api::handlers::auth_handler::AuthHandler;

/// Configure les routes d'authentification
pub fn auth_routes(handler: Arc<AuthHandler>) -> Router {
    Router::new()
        .route("/signup", post(AuthHandler::signup))
        .route("/login", post(AuthHandler::login))
        .route("/logout", post(AuthHandler::logout))
        .route("/me", get(AuthHandler::get_me))
        .with_state(handler)
}
