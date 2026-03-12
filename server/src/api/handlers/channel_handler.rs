use crate::application::use_cases::channel::*;
use crate::infrastructure::repositories::{ChannelRepository, ServerRepository};
use crate::infrastructure::security::JWTService;
use crate::utils::error::AppError;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct ChannelHandler<CR: ChannelRepository, SR: ServerRepository> {
    jwt_service: Arc<JWTService>,
    get_channel_info_uc: Arc<GetChannelInfoUseCase<CR, SR>>,
    update_channel_uc: Arc<UpdateChannelUseCase<CR, SR>>,
    delete_channel_uc: Arc<DeleteChannelUseCase<CR, SR>>,
}

impl<CR: ChannelRepository, SR: ServerRepository> ChannelHandler<CR, SR> {
    pub fn new(jwt_service: JWTService, channel_repo: CR, server_repo: SR) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service),
            get_channel_info_uc: Arc::new(GetChannelInfoUseCase::new(
                channel_repo.clone(),
                server_repo.clone(),
            )),
            update_channel_uc: Arc::new(UpdateChannelUseCase::new(
                channel_repo.clone(),
                server_repo.clone(),
            )),
            delete_channel_uc: Arc::new(DeleteChannelUseCase::new(channel_repo, server_repo)),
        }
    }
}

/// - Obtenir les infos d'un channel donné
#[utoipa::path(
    get,
    path = "/channels/{id}",
    tag = "channels",
    responses(
        (status = 200, description = "Informations du channel", body = ChannelResponse),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Channel non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_channel_info<CR: ChannelRepository, SR: ServerRepository>(
    State(handler): State<Arc<ChannelHandler<CR, SR>>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            AppError::Unauthorized("Missing or invalid Authorization header".to_string())
        })?;

    let claims = handler.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let channel = handler.get_channel_info_uc.execute(id, user_id).await?;
    Ok((StatusCode::OK, Json(channel)))
}

/// - Update les infos d'un channel
#[utoipa::path(
    put,
    path = "/channels/{id}",
    tag = "channels",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Channel modifié", body = ChannelResponse),
        (status = 401, description = "Non autorisé"),
        (status = 403, description = "Accès refusé"),
        (status = 404, description = "Channel non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_channel<CR: ChannelRepository, SR: ServerRepository>(
    State(handler): State<Arc<ChannelHandler<CR, SR>>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            AppError::Unauthorized("Missing or invalid Authorization header".to_string())
        })?;

    let claims = handler.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ValidationError("Missing name field".to_string()))?
        .to_string();

    let channel = handler.update_channel_uc.execute(id, user_id, name).await?;
    Ok((StatusCode::OK, Json(channel)))
}

/// - Supprimer un channel
#[utoipa::path(
    delete,
    path = "/channels/{id}",
    tag = "channels",
    responses(
        (status = 200, description = "Channel supprimé"),
        (status = 401, description = "Non autorisé"),
        (status = 403, description = "Accès refusé"),
        (status = 404, description = "Channel non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_channel<CR: ChannelRepository, SR: ServerRepository>(
    State(handler): State<Arc<ChannelHandler<CR, SR>>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            AppError::Unauthorized("Missing or invalid Authorization header".to_string())
        })?;

    let claims = handler.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    handler.delete_channel_uc.execute(id, user_id).await?;
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({"message": "Channel deleted"})),
    ))
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Channel;
    use crate::domain::value_objects::ServerRole;
    use crate::infrastructure::repositories::mocks::{
        mock_channel_repository::MockChannelRepository,
        mock_server_repository::MockServerRepository,
    };

    #[tokio::test]
    async fn test_get_channel_info_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ChannelHandler::new(
            jwt_service.clone(),
            mock_channel_repo,
            mock_server_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result =
            ChannelHandler::get_channel_info(State(handler), Path(channel.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_channel_info_missing_token() {
        let channel_id = Uuid::new_v4();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ChannelHandler::new(
            jwt_service,
            mock_channel_repo,
            mock_server_repo,
        ));
        let headers = HeaderMap::new();

        let result =
            ChannelHandler::get_channel_info(State(handler), Path(channel_id), headers).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_channel_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "Old Name".to_string());

        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Admin);
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ChannelHandler::new(
            jwt_service.clone(),
            mock_channel_repo,
            mock_server_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"name": "New Name"});

        let result = ChannelHandler::update_channel(
            State(handler),
            Path(channel.id),
            headers,
            Json(payload),
        )
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_channel_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "Test Channel".to_string());

        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Owner);
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ChannelHandler::new(
            jwt_service.clone(),
            mock_channel_repo,
            mock_server_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result =
            ChannelHandler::delete_channel(State(handler), Path(channel.id), headers).await;
        assert!(result.is_ok());
    }
}
