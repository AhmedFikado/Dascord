use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::infrastructure::security::JWTService;
use crate::utils::error::AppError;

/// Handler contenant la logique métier pour les serveurs
#[derive(Clone)]
pub struct ServerHandler {
    jwt_service: Arc<JWTService>,
}

impl ServerHandler {
    pub fn new(jwt_service: JWTService) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service),
        }
    }

    /// POST /servers - Créer un nouveau serveur
    pub async fn create_server(
        State(handler): State<Arc<ServerHandler>>,
        headers: HeaderMap,
        Json(payload): Json<serde_json::Value>,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Implémenter la logique de création de serveur
        // let server = handler.server_uc.create_server(claims.sub_id, payload).await?;
        
        Ok((StatusCode::CREATED, Json(serde_json::json!({
            "message": "Server created",
            "owner_id": claims.sub_id
        }))))
    }

    /// GET /servers - Obtenir la liste des serveurs
    pub async fn get_user_servers(
        State(handler): State<Arc<ServerHandler>>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Récupérer les serveurs de l'utilisateur depuis la DB
        // let servers = handler.server_uc.get_user_servers(claims.sub_id).await?;
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "user_id": claims.sub_id,
            "servers": []
        }))))
    }

    /// GET /servers/:id - Obtenir les infos d'un serveur spécifique
    pub async fn get_server_info(
        State(handler): State<Arc<ServerHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Récupérer les infos du serveur depuis la DB
        // let server = handler.server_uc.get_server_by_id(id).await?;
        // Vérifier que l'utilisateur a accès à ce serveur
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "server_id": id.to_string(),
            "user_id": claims.sub_id,
            "name": "Server Name",
            "owner_id": "owner-uuid",
            "created_at": "2024-01-01T00:00:00Z"
        }))))
    }

    /// PUT /servers/:id - Mettre à jour un serveur
    pub async fn update_server(
        State(handler): State<Arc<ServerHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
        Json(payload): Json<serde_json::Value>,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Mettre à jour le serveur
        // Vérifier que l'utilisateur est owner/admin
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "message": "Server updated",
            "server_id": id.to_string(),
            "user_id": claims.sub_id
        }))))
    }

    /// DELETE /servers/:id - Supprimer un serveur
    pub async fn delete_server(
        State(handler): State<Arc<ServerHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Supprimer le serveur
        // Vérifier que l'utilisateur est owner
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "message": "Server deleted",
            "server_id": id.to_string()
        }))))
    }

    /// POST /servers/:id/join - Rejoindre un serveur
    pub async fn join_server(
        State(handler): State<Arc<ServerHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Ajouter l'utilisateur au serveur
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "message": "Joined server",
            "server_id": id.to_string(),
            "user_id": claims.sub_id
        }))))
    }

    /// DELETE /servers/:id/leave - Quitter un serveur
    pub async fn leave_server(
        State(handler): State<Arc<ServerHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Retirer l'utilisateur du serveur
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "message": "Left server",
            "server_id": id.to_string(),
            "user_id": claims.sub_id
        }))))
    }

    /// GET /servers/:id/members - Lister les membres d'un serveur
    pub async fn list_members(
        State(handler): State<Arc<ServerHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Récupérer la liste des membres
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "server_id": id.to_string(),
            "members": []
        }))))
    }

    /// PUT /servers/:id/members/:userId - Mettre à jour le rôle d'un membre
    pub async fn update_member_role(
        State(handler): State<Arc<ServerHandler>>,
        Path((id, user_id)): Path<(Uuid, Uuid)>,
        headers: HeaderMap,
        Json(payload): Json<serde_json::Value>,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Mettre à jour le rôle du membre
        // Vérifier que l'utilisateur est owner/admin
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "message": "Member role updated",
            "server_id": id.to_string(),
            "user_id": user_id.to_string()
        }))))
    }

    /// GET /servers/:id/channels - Obtenir la liste des canaux d'un serveur
    pub async fn get_channels(
        State(handler): State<Arc<ServerHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Récupérer les canaux du serveur depuis la DB
        // let channels = handler.channel_uc.get_server_channels(id).await?;
        // Vérifier que l'utilisateur a accès à ce serveur
        
        Ok((StatusCode::OK, Json(serde_json::json!({
            "server_id": id.to_string(),
            "user_id": claims.sub_id,
            "channels": []
        }))))
    }

    /// POST /servers/:id/channels - Créer un nouveau canal dans un serveur
    pub async fn create_channel(
        State(handler): State<Arc<ServerHandler>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
        Json(payload): Json<serde_json::Value>,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        
        // TODO: Créer le canal
        // let channel = handler.channel_uc.create_channel(id, payload).await?;
        // Vérifier que l'utilisateur est owner/admin du serveur
        
        Ok((StatusCode::CREATED, Json(serde_json::json!({
            "message": "Channel created",
            "server_id": id.to_string(),
            "user_id": claims.sub_id
        }))))
    }
}
