use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub postgres_url: String,
    pub mongodb_url: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            postgres_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5433/rtc_db".to_string()),
            mongodb_url: std::env::var("MONGODB_URI")
                .or_else(|_| std::env::var("MONGODB_URL"))
                .unwrap_or_else(|_| "mongodb://admin:admin@localhost:27017".to_string()),
        }
    }
}
