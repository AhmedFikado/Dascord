use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    // Le token sera extrait du header Authorization, pas du body
}
