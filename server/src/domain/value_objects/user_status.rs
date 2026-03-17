use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserStatus {
    Online,
    Offline,
    Inactive,
    #[serde(rename = "DO NOT DISTURB")]
    DoNotDisturb,
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_status_equality() {
        assert_eq!(UserStatus::Online, UserStatus::Online);
        assert_eq!(UserStatus::Offline, UserStatus::Offline);
        assert_eq!(UserStatus::Inactive, UserStatus::Inactive);
        assert_eq!(UserStatus::DoNotDisturb, UserStatus::DoNotDisturb);
        assert_ne!(UserStatus::Online, UserStatus::Offline);
    }

    #[test]
    fn test_user_status_serialization() {
        let status = UserStatus::Online;
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("ONLINE"));
    }

    #[test]
    fn test_user_status_deserialization() {
        let json = r#""OFFLINE""#;
        let status: UserStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, UserStatus::Offline);
    }
}
