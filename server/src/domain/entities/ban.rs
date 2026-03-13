use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "ban_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BanType {
    Temporary,
    Permanent,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ban {
    pub id: Uuid,
    pub server_id: Uuid,
    pub user_id: Uuid,
    pub banned_by: Uuid,
    pub ban_type: BanType,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Ban {
    pub fn new(server_id: Uuid, user_id: Uuid, banned_by: Uuid, ban_type: BanType, expires_at: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            server_id,
            user_id,
            banned_by,
            ban_type,
            expires_at,
            created_at: chrono::Utc::now(),
        }
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let banned_by = Uuid::new_v4();
        let ban_type = BanType::Temporary;
        let expires_at = Some(chrono::Utc::now());

        let server = Ban::new(server_id, user_id, banned_by, ban_type, expires_at);

        assert_eq!(server.server_id, server_id);
        assert_eq!(server.user_id, user_id);
        assert_eq!(server.banned_by, banned_by);
        assert_eq!(server.ban_type, ban_type);
        assert_eq!(server.expires_at, expires_at);
    }

    #[test]
    fn test_server_has_unique_id() {
        let server_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let banned_by = Uuid::new_v4();
        let ban_type = BanType::Temporary;
        let expires_at = Some(chrono::Utc::now());

        let server1 = Ban::new(server_id, user_id, banned_by, ban_type, expires_at);
        let server2 = Ban::new(server_id, user_id, banned_by, ban_type, expires_at);

        assert_ne!(server1.id, server2.id);
    }
}
