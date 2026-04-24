use crate::application::dto::server_dto::CreateServerRequest;
use crate::application::dto::server_dto::JoinServerRequest;
use crate::domain::services::server::*;
use crate::domain::entities::BanType;
use crate::domain::value_objects::ServerRole;
use crate::infrastructure::repositories::{ServerRepository, UserRepository};
use crate::infrastructure::repositories::channel::ChannelRepository;
use crate::infrastructure::security::JWTService;
use crate::infrastructure::websocket::ConnectionManager;
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
pub struct ServerHandler<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository> {
    jwt_service: Arc<JWTService>,
    create_server_uc: Arc<CreateServerUseCase<SR>>,
    get_user_servers_uc: Arc<GetUserServersUseCase<SR>>,
    get_server_info_uc: Arc<GetServerInfoUseCase<SR>>,
    update_server_uc: Arc<UpdateServerUseCase<SR>>,
    delete_server_uc: Arc<DeleteServerUseCase<SR>>,
    join_server_uc: Arc<JoinServerUseCase<SR>>,
    leave_server_uc: Arc<LeaveServerUseCase<SR>>,
    list_members_uc: Arc<ListMembersUseCase<SR, UR>>,
    update_member_role_uc: Arc<UpdateMemberRoleUseCase<SR>>,
    kick_member_uc: Arc<KickMemberUseCase<SR>>,
    ban_member_uc: Arc<BanMemberUseCase<SR>>,
    list_banned_members_uc: Arc<ListBannedMembersUseCase<SR>>,
    unban_member_uc: Arc<UnbanMemberUseCase<SR>>,
    get_channels_uc: Arc<GetChannelsUseCase<SR, CR>>,
    create_channel_uc: Arc<CreateChannelUseCase<SR, CR>>,
    user_repo: Arc<UR>,
    ws_manager: Option<Arc<ConnectionManager>>,
}

impl<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository> ServerHandler<SR, CR, UR> {
    pub fn new(
        jwt_service: JWTService,
        server_repo: SR,
        channel_repo: CR,
        user_repo: UR,
    ) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service),
            create_server_uc: Arc::new(CreateServerUseCase::new(server_repo.clone())),
            get_user_servers_uc: Arc::new(GetUserServersUseCase::new(server_repo.clone())),
            get_server_info_uc: Arc::new(GetServerInfoUseCase::new(server_repo.clone())),
            update_server_uc: Arc::new(UpdateServerUseCase::new(server_repo.clone())),
            delete_server_uc: Arc::new(DeleteServerUseCase::new(server_repo.clone())),
            join_server_uc: Arc::new(JoinServerUseCase::new(server_repo.clone())),
            leave_server_uc: Arc::new(LeaveServerUseCase::new(server_repo.clone())),
            list_members_uc: Arc::new(ListMembersUseCase::new(server_repo.clone(), user_repo.clone())),
            update_member_role_uc: Arc::new(UpdateMemberRoleUseCase::new(server_repo.clone())),
            kick_member_uc: Arc::new(KickMemberUseCase::new(server_repo.clone())),
            ban_member_uc: Arc::new(BanMemberUseCase::new(server_repo.clone())),
            list_banned_members_uc: Arc::new(ListBannedMembersUseCase::new(server_repo.clone())),
            unban_member_uc: Arc::new(UnbanMemberUseCase::new(server_repo.clone())),
            get_channels_uc: Arc::new(GetChannelsUseCase::new(
                server_repo.clone(),
                channel_repo.clone(),
            )),
            create_channel_uc: Arc::new(CreateChannelUseCase::new(server_repo, channel_repo)),
            user_repo: Arc::new(user_repo),
            ws_manager: None,
        }
    }

    pub fn with_ws_manager(mut self, ws_manager: Arc<ConnectionManager>) -> Self {
        self.ws_manager = Some(ws_manager);
        self
    }
}

/// - Créer un nouveau serveur
#[utoipa::path(
    post,
    path = "/servers",
    tag = "servers",
    request_body = CreateServerRequest,
    responses(
        (status = 201, description = "Serveur créé avec succès", body = ServerResponse),
        (status = 401, description = "Non autorisé"),
        (status = 400, description = "Erreur de validation")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_server<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
    headers: HeaderMap,
    Json(request): Json<CreateServerRequest>,
) -> Result<impl IntoResponse, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            AppError::Unauthorized("Missing or invalid Authorization header".to_string())
        })?;

    let claims = handler.jwt_service.verify_token(token)?;
    let owner_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let response = handler.create_server_uc.execute(request, owner_id).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

/// - Obtenir la liste des serveurs dans lequel se trouve l'utilisateur
#[utoipa::path(
    get,
    path = "/servers",
    tag = "servers",
    responses(
        (status = 200, description = "Liste des serveurs", body = Vec<ServerResponse>),
        (status = 401, description = "Non autorisé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_user_servers<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
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

    let servers = handler.get_user_servers_uc.execute(user_id).await?;
    Ok((StatusCode::OK, Json(servers)))
}

/// - Obtenir les informations d'un serveur
#[utoipa::path(
    get,
    path = "/servers/{id}",
    tag = "servers",
    responses(
        (status = 200, description = "Informations du serveur", body = ServerResponse),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Serveur non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_server_info<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
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

    let server = handler.get_server_info_uc.execute(id, user_id).await?;
    Ok((StatusCode::OK, Json(server)))
}

/// - Mettre à jour les informations d'un serveur
#[utoipa::path(
    put,
    path = "/servers/{id}",
    tag = "servers",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Serveur mis à jour", body = ServerResponse),
        (status = 401, description = "Non autorisé"),
        (status = 403, description = "Accès refusé"),
        (status = 404, description = "Serveur non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_server<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
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

    let server = handler.update_server_uc.execute(id, user_id, name).await?;

    if let Some(ws_manager) = &handler.ws_manager {
        ws_manager.broadcast_to_all(
            crate::infrastructure::websocket::ServerMessage::ServerNameUpdated {
                server_id: server.id.clone(),
                name: server.name.clone(),
            },
        ).await;
    }

    Ok((StatusCode::OK, Json(server)))
}

/// - Supprimer un serveur
#[utoipa::path(
    delete,
    path = "/servers/{id}",
    tag = "servers",
    responses(
        (status = 200, description = "Serveur supprimé"),
        (status = 401, description = "Non autorisé"),
        (status = 403, description = "Accès refusé"),
        (status = 404, description = "Serveur non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_server<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
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

    handler.delete_server_uc.execute(id, user_id).await?;
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({"message": "Server deleted"})),
    ))
}

/// - Rejoindre un serveur
#[utoipa::path(
    post,
    path = "/servers/join",
    tag = "servers",
    request_body = JoinServerRequest,
    responses(
        (status = 200, description = "Serveur rejoint avec succès"),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Code d'invitation invalide"),
        (status = 409, description = "Déjà membre du serveur")
    ),
    security(("bearer_auth" = []))
)]
pub async fn join_server<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
    headers: HeaderMap,
    Json(request): Json<JoinServerRequest>,
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

    let server_id = handler
        .join_server_uc
        .execute(&request.invitation_code, user_id)
        .await?;

    // Récupérer les infos de l'utilisateur pour la diffusion
    let user = handler
        .user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Diffuser l'événement WebSocket à tous les clients connectés
    if let Some(ws_manager) = &handler.ws_manager {
        ws_manager.broadcast_to_all(
            crate::infrastructure::websocket::ServerMessage::ServerMemberJoined {
                server_id: server_id.to_string(),
                user_id: user_id.to_string(),
                username: user.username,
            },
        ).await;
    }

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({"message": "Joined server"})),
    ))
}

/// - Quitter un serveur
#[utoipa::path(
    delete,
    path = "/servers/{id}/leave",
    tag = "servers",
    responses(
        (status = 200, description = "Serveur quitté avec succès"),
        (status = 401, description = "Non autorisé"),
        (status = 400, description = "Le propriétaire ne peut pas quitter"),
        (status = 404, description = "Serveur non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn leave_server<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
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

    handler.leave_server_uc.execute(id, user_id).await?;

    // Diffuser l'événement WebSocket à tous les clients connectés
    if let Some(ws_manager) = &handler.ws_manager {
        ws_manager.broadcast_to_all(
            crate::infrastructure::websocket::ServerMessage::ServerMemberLeft {
                server_id: id.to_string(),
                user_id: user_id.to_string(),
            },
        ).await;
    }

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({"message": "Left server"})),
    ))
}

/// - Obtenir la liste des membres d'un serveur
#[utoipa::path(
    get,
    path = "/servers/{id}/members",
    tag = "servers",
    responses(
        (status = 200, description = "Liste des membres", body = Vec<MemberResponse>),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Serveur non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_members<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
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

    let members = handler.list_members_uc.execute(id, user_id).await?;
    Ok((StatusCode::OK, Json(members)))
}

/// - Mettre à jour le rôle d'un membre
#[utoipa::path(
    put,
    path = "/servers/{id}/members/{userId}",
    tag = "servers",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Rôle mis à jour"),
        (status = 401, description = "Non autorisé"),
        (status = 403, description = "Permissions insuffisantes"),
        (status = 404, description = "Serveur ou membre non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_member_role<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
    Path((id, target_user_id)): Path<(Uuid, Uuid)>,
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
    let requester_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let role: ServerRole =
        serde_json::from_value(payload.get("role").cloned().unwrap_or_default())
            .map_err(|_| AppError::ValidationError("Invalid role".to_string()))?;

    handler
        .update_member_role_uc
        .execute(id, target_user_id, requester_id, role.clone())
        .await?;

    if let Some(ws_manager) = &handler.ws_manager {
        let role_string = serde_json::to_value(&role)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "MEMBER".to_string());

        ws_manager.broadcast_to_all(
            crate::infrastructure::websocket::ServerMessage::MemberRoleUpdated {
                server_id: id.to_string(),
                user_id: target_user_id.to_string(),
                new_role: role_string.clone(),
            },
        ).await;

        if role == ServerRole::Owner {
            ws_manager.broadcast_to_all(
                crate::infrastructure::websocket::ServerMessage::MemberRoleUpdated {
                    server_id: id.to_string(),
                    user_id: requester_id.to_string(),
                    new_role: "ADMIN".to_string(),
                },
            ).await;
        }
    }

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({"message": "Member role updated"})),
    ))
}

/// - Expulser un membre d'un serveur
#[utoipa::path(
    delete,
    path = "/servers/{id}/members/{userId}/kick",
    tag = "servers",
    responses(
        (status = 200, description = "Membre expulsé"),
        (status = 401, description = "Non autorisé"),
        (status = 403, description = "Permissions insuffisantes"),
        (status = 404, description = "Serveur ou membre non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn kick_member<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
    Path((id, target_user_id)): Path<(Uuid, Uuid)>,
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
    let requester_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    handler.kick_member_uc.execute(id, target_user_id, requester_id).await?;

    if let Some(ws_manager) = &handler.ws_manager {
        ws_manager.broadcast_to_all(
            crate::infrastructure::websocket::ServerMessage::MemberKicked {
                server_id: id.to_string(),
                user_id: target_user_id.to_string(),
            },
        ).await;
    }

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({"message": "Member kicked"})),
    ))
}

/// - Bannir un membre d'un serveur
#[utoipa::path(
    post,
    path = "/servers/{id}/members/{userId}/ban",
    tag = "servers",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Membre banni"),
        (status = 401, description = "Non autorisé"),
        (status = 403, description = "Permissions insuffisantes"),
        (status = 404, description = "Serveur ou membre non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn ban_member<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
    Path((id, target_user_id)): Path<(Uuid, Uuid)>,
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
    let requester_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let ban_type: BanType =
        serde_json::from_value(payload.get("ban_type").cloned().unwrap_or_default())
            .map_err(|_| AppError::ValidationError("Invalid ban_type".to_string()))?;

    let expires_at: Option<chrono::DateTime<chrono::Utc>> = payload
        .get("expires_at")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    handler.ban_member_uc.execute(id, target_user_id, requester_id, ban_type, expires_at).await?;

    if let Some(ws_manager) = &handler.ws_manager {
        ws_manager.broadcast_to_all(
            crate::infrastructure::websocket::ServerMessage::MemberBanned {
                server_id: id.to_string(),
                user_id: target_user_id.to_string(),
            },
        ).await;
    }

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({"message": "Member banned"})),
    ))
}

/// - Lister les membres bannis d'un serveur
#[utoipa::path(
    get,
    path = "/servers/{id}/bans",
    tag = "servers",
    responses(
        (status = 200, description = "Liste des membres bannis", body = Vec<crate::application::dto::server_dto::BanResponse>),
        (status = 401, description = "Non autorisé"),
        (status = 403, description = "Permissions insuffisantes"),
        (status = 404, description = "Serveur non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_banned_members<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
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
    let requester_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let bans = handler.list_banned_members_uc.execute(id, requester_id).await?;

    let response: Vec<crate::application::dto::server_dto::BanResponse> = bans.into_iter().map(|(user_id, username, ban_type, banned_at, expires_at)| {
        crate::application::dto::server_dto::BanResponse {
            user_id: user_id.to_string(),
            username,
            ban_type: format!("{:?}", ban_type),
            banned_at: banned_at.to_rfc3339(),
            expires_at: expires_at.map(|e| e.to_rfc3339()),
        }
    }).collect();

    Ok((StatusCode::OK, Json(response)))
}

/// - Débannir un membre d'un serveur
#[utoipa::path(
    delete,
    path = "/servers/{id}/members/{userId}/ban",
    tag = "servers",
    responses(
        (status = 200, description = "Membre débanni"),
        (status = 401, description = "Non autorisé"),
        (status = 403, description = "Permissions insuffisantes"),
        (status = 404, description = "Serveur ou membre non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn unban_member<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
    Path((id, target_user_id)): Path<(Uuid, Uuid)>,
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
    let requester_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    handler.unban_member_uc.execute(id, target_user_id, requester_id).await?;

    if let Some(ws_manager) = &handler.ws_manager {
        ws_manager.broadcast_to_all(
            crate::infrastructure::websocket::ServerMessage::MemberUnbanned {
                server_id: id.to_string(),
                user_id: target_user_id.to_string(),
            },
        ).await;
    }

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({"message": "Member unbanned"})),
    ))
}

/// - Obtenir les channels d'un serveur
#[utoipa::path(
    get,
    path = "/servers/{id}/channels",
    tag = "servers",
    responses(
        (status = 200, description = "Liste des channels", body = Vec<ChannelResponse>),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Serveur non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_channels<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
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

    let channels = handler.get_channels_uc.execute(id, user_id).await?;
    Ok((StatusCode::OK, Json(channels)))
}

/// - Créer un channel dans un serveur
#[utoipa::path(
    post,
    path = "/servers/{id}/channels",
    tag = "servers",
    request_body = CreateChannelRequest,
    responses(
        (status = 201, description = "Channel créé avec succès", body = ChannelResponse),
        (status = 401, description = "Non autorisé"),
        (status = 403, description = "Permissions insuffisantes"),
        (status = 404, description = "Serveur non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_channel<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository>(
    State(handler): State<Arc<ServerHandler<SR, CR, UR>>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(request): Json<crate::application::dto::channel::CreateChannelRequest>,
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

    let channel = handler
        .create_channel_uc
        .execute(id, user_id, request)
        .await?;
    Ok((StatusCode::CREATED, Json(channel)))
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::channel::CreateChannelRequest;
    use crate::application::dto::server_dto::{CreateServerRequest, JoinServerRequest};
    use crate::domain::entities::Server;
    use crate::domain::value_objects::ServerRole;
    use crate::infrastructure::repositories::mocks::{
        mock_channel_repository::MockChannelRepository,
        mock_server_repository::MockServerRepository,
        mock_user_repository::MockUserRepository,
    };

    #[tokio::test]
    async fn test_create_server_success() {
        let owner_id = Uuid::new_v4();

        let mock_server_repo = MockServerRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
            mock_user_repo,
        ));

        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let request = CreateServerRequest {
            name: "Test Server".to_string(),
        };

        let result = create_server(State(handler), headers, Json(request)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_servers_success() {
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), user_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Owner);
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result = get_user_servers(State(handler), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_join_server_success() {
        use crate::domain::entities::User;
        use crate::infrastructure::security::PasswordService;

        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let password_service = PasswordService::new();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@test.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            language: "en".to_string(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
            avatar_id: None,
        };

        let mock_server_repo = MockServerRepository::new().with_server(server.clone());
        let mock_channel_repo = MockChannelRepository::new();
        let mock_user_repo = MockUserRepository::new().with_user(user);
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
            mock_user_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let request = JoinServerRequest {
            invitation_code: "CODE123".to_string(),
        };

        let result = join_server(State(handler), headers, Json(request)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_channel_success() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new().with_server(server.clone());
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));

        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let request = CreateChannelRequest {
            name: "General".to_string(),
        };

        let result =
            create_channel(State(handler), Path(server.id), headers, Json(request))
                .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_missing_authorization_header() {
        let mock_server_repo = MockServerRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service,
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let headers = HeaderMap::new();

        let result = get_user_servers(State(handler), headers).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_server_info_success() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, owner_id, ServerRole::Owner);
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result = get_server_info(State(handler), Path(server.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_server_success() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Old Name".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new().with_server(server.clone());
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"name": "New Name"});
        let result =
            update_server(State(handler), Path(server.id), headers, Json(payload)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_server_success() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new().with_server(server.clone());
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result = delete_server(State(handler), Path(server.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_leave_server_success() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Member);
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result = leave_server(State(handler), Path(server.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_members_success() {
        use crate::domain::entities::User;
        use crate::infrastructure::security::PasswordService;

        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let password_service = PasswordService::new();
        let owner = User {
            id: owner_id,
            username: "owner".to_string(),
            email: "owner@test.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            language: "en".to_string(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
            avatar_id: None,
        };

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, owner_id, ServerRole::Owner);
        let mock_channel_repo = MockChannelRepository::new();
        let mock_user_repo = MockUserRepository::new().with_user(owner);
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
            mock_user_repo,
        ));
        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result = list_members(State(handler), Path(server.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_member_role_success() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Member);
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"role": "ADMIN"});
        let result = update_member_role(
            State(handler),
            Path((server.id, user_id)),
            headers,
            Json(payload),
        )
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_channels_success() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, owner_id, ServerRole::Owner);
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result = get_channels(State(handler), Path(server.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_kick_member_success() {
        let owner_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, member_id, ServerRole::Member);
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
            MockUserRepository::new(),
        ));

        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        let result = kick_member(State(handler), Path((server.id, member_id)), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_ban_member_success() {
        let owner_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, member_id, ServerRole::Member);
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
            MockUserRepository::new(),
        ));

        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        let payload = serde_json::json!({"ban_type": "Permanent"});
        let result = ban_member(State(handler), Path((server.id, member_id)), headers, Json(payload)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_token() {
        let mock_server_repo = MockServerRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service,
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer invalid_token".parse().unwrap());

        let result = get_user_servers(State(handler), headers).await;
        assert!(result.is_err());
    }
}

