use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::domain::entities::Server;
use crate::domain::value_objects::ServerRole;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateServerRequest {
    #[validate(length(min = 1, max = 100, message = "Server name must be between 1 and 100 characters"))]
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ServerResponse {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub invitation_code: String,
    pub created_at: String,
}

impl From<Server> for ServerResponse {
    fn from(server: Server) -> Self {
        Self {
            id: server.id.to_string(),
            name: server.name,
            owner_id: server.owner_id.to_string(),
            invitation_code: server.invitation_code,
            created_at: server.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub user_id: String,
    pub role: ServerRole,
}

impl From<(Uuid, ServerRole)> for MemberResponse {
    fn from((user_id, role): (Uuid, ServerRole)) -> Self {
        Self {
            user_id: user_id.to_string(),
            role,
        }
    }
}
