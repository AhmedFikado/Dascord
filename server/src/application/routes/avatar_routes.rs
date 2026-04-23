use crate::application::controller::avatar_controller::{download_avatar, upload_avatar, AvatarHandler};
use crate::infrastructure::repositories::UserRepository;
use axum::{routing::{get, post}, Router};
use std::sync::Arc;

pub fn avatar_routes<R: UserRepository + 'static>(handler: Arc<AvatarHandler<R>>) -> Router {
    Router::new()
        .route("/users/avatar", post(upload_avatar::<R>))
        .route("/avatars/:avatar_id", get(download_avatar::<R>))
        .with_state(handler)
}
