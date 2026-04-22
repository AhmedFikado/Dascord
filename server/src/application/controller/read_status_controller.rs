use crate::application::dto::read_status_dto::{UnreadChannelDto, UnreadStatusResponse};
use crate::infrastructure::repositories::read_status_repository::ReadStatusRepository;
use crate::infrastructure::repositories::channel::ChannelRepository;
use crate::infrastructure::repositories::ServerRepository;
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
pub struct ReadStatusHandler<RSR: ReadStatusRepository, CR: ChannelRepository, SR: ServerRepository> {
    jwt_service: Arc<JWTService>,
    read_status_repo: RSR,
    channel_repo: CR,
    server_repo: SR,
}

impl<RSR: ReadStatusRepository, CR: ChannelRepository, SR: ServerRepository>
    ReadStatusHandler<RSR, CR, SR>
{
    pub fn new(
        jwt_service: JWTService,
        read_status_repo: RSR,
        channel_repo: CR,
        server_repo: SR,
    ) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service),
            read_status_repo,
            channel_repo,
            server_repo,
        }
    }
}

fn extract_user_id(headers: &HeaderMap, jwt_service: &JWTService) -> Result<Uuid, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;
    let claims = jwt_service.verify_token(token)?;
    Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))
}

/// Marquer un channel comme lu
#[utoipa::path(
    post,
    path = "/channels/{id}/read",
    tag = "channels",
    responses(
        (status = 200, description = "Channel marqué comme lu"),
        (status = 401, description = "Non autorisé"),
        (status = 403, description = "Accès refusé"),
        (status = 404, description = "Channel non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn mark_channel_read<
    RSR: ReadStatusRepository,
    CR: ChannelRepository,
    SR: ServerRepository,
>(
    State(handler): State<Arc<ReadStatusHandler<RSR, CR, SR>>>,
    Path(channel_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let user_id = extract_user_id(&headers, &handler.jwt_service)?;

    let channel = handler
        .channel_repo
        .find_by_id(channel_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

    let is_member = handler
        .server_repo
        .is_member(channel.server_id, user_id)
        .await?;
    if !is_member {
        return Err(AppError::Unauthorized("Not a member of this server".to_string()));
    }

    handler
        .read_status_repo
        .mark_read(user_id, channel_id)
        .await?;

    Ok((StatusCode::OK, Json(serde_json::json!({"message": "Channel marked as read"}))))
}

/// Obtenir les channels non lus d'un serveur
#[utoipa::path(
    get,
    path = "/servers/{id}/unread",
    tag = "servers",
    responses(
        (status = 200, description = "Channels non lus du serveur", body = UnreadStatusResponse),
        (status = 401, description = "Non autorisé"),
        (status = 403, description = "Accès refusé"),
        (status = 404, description = "Serveur non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_unread_channels<
    RSR: ReadStatusRepository,
    CR: ChannelRepository,
    SR: ServerRepository,
>(
    State(handler): State<Arc<ReadStatusHandler<RSR, CR, SR>>>,
    Path(server_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let user_id = extract_user_id(&headers, &handler.jwt_service)?;

    let is_member = handler.server_repo.is_member(server_id, user_id).await?;
    if !is_member {
        return Err(AppError::Unauthorized("Not a member of this server".to_string()));
    }

    let unread = handler
        .read_status_repo
        .get_unread_channels_for_server(user_id, server_id)
        .await?;

    let response = UnreadStatusResponse {
        unread_channels: unread
            .into_iter()
            .map(|info| UnreadChannelDto {
                channel_id: info.channel_id,
                first_unread_message_id: info.first_unread_message_id,
            })
            .collect(),
    };

    Ok((StatusCode::OK, Json(response)))
}


/// Obtenir les conversations privées non lues
#[utoipa::path(
    get,
    path = "/channels/private/unread",
    tag = "channels",
    responses(
        (status = 200, description = "Conversations privées non lues", body = UnreadStatusResponse),
        (status = 401, description = "Non autorisé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_unread_private_channels<
    RSR: ReadStatusRepository,
    CR: ChannelRepository,
    SR: ServerRepository,
>(
    State(handler): State<Arc<ReadStatusHandler<RSR, CR, SR>>>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let user_id = extract_user_id(&headers, &handler.jwt_service)?;

    let unread = handler
        .read_status_repo
        .get_unread_private_channels(user_id)
        .await?;

    let response = UnreadStatusResponse {
        unread_channels: unread
            .into_iter()
            .map(|info| UnreadChannelDto {
                channel_id: info.channel_id,
                first_unread_message_id: info.first_unread_message_id,
            })
            .collect(),
    };

    Ok((StatusCode::OK, Json(response)))
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::channel::Channel;
    use crate::domain::value_objects::ServerRole;
    use crate::infrastructure::repositories::mocks::{
        mock_channel_repository::MockChannelRepository,
        mock_read_status_repository::MockReadStatusRepository,
        mock_server_repository::MockServerRepository,
    };

    fn make_handler(
        channel: Option<Channel>,
        server_id: Option<Uuid>,
        user_id: Option<Uuid>,
    ) -> Arc<ReadStatusHandler<MockReadStatusRepository, MockChannelRepository, MockServerRepository>> {
        let jwt_service = crate::infrastructure::security::JWTService::new("test_secret".to_string());
        let mut channel_repo = MockChannelRepository::new();
        let mut server_repo = MockServerRepository::new();
        if let Some(ch) = channel {
            channel_repo = channel_repo.with_channel(ch);
        }
        if let (Some(sid), Some(uid)) = (server_id, user_id) {
            server_repo = server_repo.with_member(sid, uid, ServerRole::Member);
        }
        Arc::new(ReadStatusHandler::new(
            jwt_service,
            MockReadStatusRepository::new(),
            channel_repo,
            server_repo,
        ))
    }

    #[tokio::test]
    async fn test_mark_channel_read_missing_token() {
        let handler = make_handler(None, None, None);
        let result = mark_channel_read(
            State(handler),
            Path(Uuid::new_v4()),
            HeaderMap::new(),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mark_channel_read_channel_not_found() {
        let user_id = Uuid::new_v4();
        let jwt_service = crate::infrastructure::security::JWTService::new("test_secret".to_string());
        let token = jwt_service.create_token(user_id).unwrap();
        let handler = make_handler(None, None, None);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        let result = mark_channel_read(State(handler), Path(Uuid::new_v4()), headers).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mark_channel_read_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "general".to_string());
        let channel_id = channel.id;

        let jwt_service = crate::infrastructure::security::JWTService::new("test_secret".to_string());
        let token = jwt_service.create_token(user_id).unwrap();
        let handler = make_handler(Some(channel), Some(server_id), Some(user_id));

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        let result = mark_channel_read(State(handler), Path(channel_id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_unread_channels_missing_token() {
        let handler = make_handler(None, None, None);
        let result = get_unread_channels(
            State(handler),
            Path(Uuid::new_v4()),
            HeaderMap::new(),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_unread_channels_not_member() {
        let user_id = Uuid::new_v4();
        let jwt_service = crate::infrastructure::security::JWTService::new("test_secret".to_string());
        let token = jwt_service.create_token(user_id).unwrap();
        let handler = make_handler(None, None, None);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        let result = get_unread_channels(State(handler), Path(Uuid::new_v4()), headers).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_unread_channels_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();

        let jwt_service = crate::infrastructure::security::JWTService::new("test_secret".to_string());
        let token = jwt_service.create_token(user_id).unwrap();
        let handler = make_handler(None, Some(server_id), Some(user_id));

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        let result = get_unread_channels(State(handler), Path(server_id), headers).await;
        assert!(result.is_ok());
    }
}
