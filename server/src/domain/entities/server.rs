use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub invitation_code: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Server {
    pub fn new(name: String, owner_id: Uuid, invitation_code: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            owner_id,
            invitation_code,
            created_at: chrono::Utc::now(),
        }
    }
}
