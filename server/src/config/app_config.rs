use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("SERVER_PORT doit être un nombre valide"),
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_format() {
        let config = AppConfig {
            server_host: "127.0.0.1".to_string(),
            server_port: 8080,
        };

        assert_eq!(config.address(), "127.0.0.1:8080");
    }

    #[test]
    fn test_address_with_different_port() {
        let config = AppConfig {
            server_host: "0.0.0.0".to_string(),
            server_port: 3000,
        };

        assert_eq!(config.address(), "0.0.0.0:3000");
    }
}
