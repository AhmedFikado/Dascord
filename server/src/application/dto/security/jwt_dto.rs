use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTToken {
    pub sub_id: String,    // User ID
    pub exp: usize,       // Expiration timestamp
    pub iat: usize,       // Issued at timestamp
}