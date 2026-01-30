use serde::{Serialize};
use super::user_response_dto::UserResponse;

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    pub user: UserResponse,
    pub token: String,
}