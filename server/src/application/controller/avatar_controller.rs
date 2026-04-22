use crate::infrastructure::repositories::UserRepository;
use crate::infrastructure::security::JWTService;
use crate::infrastructure::websocket::{ConnectionManager, ServerMessage};
use crate::utils::error::AppError;
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use base64::{engine::general_purpose, Engine as _};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AvatarHandler<R: UserRepository> {
    jwt_service: Arc<JWTService>,
    user_repo: Arc<R>,
    ws_manager: Option<Arc<ConnectionManager>>,
}

impl<R: UserRepository> AvatarHandler<R> {
    pub fn new(jwt_service: JWTService, user_repo: R) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service),
            user_repo: Arc::new(user_repo),
            ws_manager: None,
        }
    }

    pub fn with_ws_manager(mut self, ws_manager: Arc<ConnectionManager>) -> Self {
        self.ws_manager = Some(ws_manager);
        self
    }
}

/// Upload avatar
#[utoipa::path(
    post,
    path = "/users/avatar",
    tag = "users",
    request_body(content = Vec<u8>, content_type = "image/*"),
    responses(
        (status = 200, description = "Avatar uploaded", body = serde_json::Value),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn upload_avatar<R: UserRepository>(
    State(handler): State<Arc<AvatarHandler<R>>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;

    let claims = handler.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    if body.is_empty() {
        return Err(AppError::ValidationError(
            "Avatar image cannot be empty".to_string(),
        ));
    }

    let avatar_data = general_purpose::STANDARD.encode(body);

    // Update user avatar_id
    handler
        .user_repo
        .update_avatar(user_id, Some(avatar_data.clone()))
        .await?;

    if let Some(ws_manager) = &handler.ws_manager {
        ws_manager
            .broadcast_to_all(ServerMessage::UserAvatarUpdated {
                user_id: user_id.to_string(),
                avatar_id: avatar_data.clone(),
            })
            .await;
    }

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({ "avatar_id": avatar_data })),
    ))
}

/// Download avatar
#[utoipa::path(
    get,
    path = "/avatars/{avatar_id}",
    tag = "users",
    params(
        ("avatar_id" = String, Path, description = "Avatar ID (base64)")
    ),
    responses(
        (status = 200, description = "Avatar image", content_type = "image/*"),
        (status = 404, description = "Avatar not found")
    )
)]
pub async fn download_avatar<R: UserRepository>(
    State(_handler): State<Arc<AvatarHandler<R>>>,
    Path(avatar_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let image_data = general_purpose::STANDARD
        .decode(avatar_id)
        .map_err(|_| AppError::BadRequest("Invalid avatar ID".to_string()))?;

    Ok((
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "image/*")],
        image_data,
    ))
}
