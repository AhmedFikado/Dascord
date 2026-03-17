use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePrivateChannelRequest {
    pub user1: Uuid,
    pub user2: Uuid,
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, from_str};

    #[test]
    fn test_valid_create_private_channel_request() {
        let user_test1 = Uuid::new_v4();
        let user_test2 = Uuid::new_v4();
        let request = CreatePrivateChannelRequest {
            user1: user_test1,
            user2: user_test2,
        };
        assert!(request.user1 == user_test1);
        assert!(request.user2 == user_test2);
    }

    #[test]
    fn test_invalid_create_private_channel_request() {
        let json_body = json!({
            "user1": 123,
            "user2": "550e8400-e29b-41d4-a716-446655440001"
        });

        let result: Result<CreatePrivateChannelRequest, _> = from_str(&json_body.to_string());
        assert!(result.is_err());
    }
}