use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum ServerRole {
    Owner,
    Admin,
    Member,
}

impl ServerRole {
    pub fn can_have_permissions(&self) -> bool {
        matches!(self, ServerRole::Owner | ServerRole::Admin)
    }

    pub fn can_delete_server(&self) -> bool {
        matches!(self, ServerRole::Owner)
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owner_permissions() {
        let role = ServerRole::Owner;
        assert!(role.can_have_permissions());
        assert!(role.can_delete_server());
    }

    #[test]
    fn test_admin_permissions() {
        let role = ServerRole::Admin;
        assert!(role.can_have_permissions());
        assert!(!role.can_delete_server());
    }

    #[test]
    fn test_member_permissions() {
        let role = ServerRole::Member;
        assert!(!role.can_have_permissions());
        assert!(!role.can_delete_server());
    }
}
