use serde::{Serialize};
use super::user_response_dto::UserResponse;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub user: UserResponse,
    pub token: String,
}