use crate::config::DatabaseConfig;
use mongodb::{options::ClientOptions, Client as MongoClient};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
    pub mongo_client: MongoClient,
}

pub async fn init_databases(
    config: &DatabaseConfig,
) -> Result<AppState, Box<dyn std::error::Error>> {
    tracing::info!("Connexion à PostgreSQL...");
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.postgres_url)
        .await?;
    tracing::info!("PostgreSQL connecté");

    tracing::info!("Connexion à MongoDB...");
    let mongo_options = ClientOptions::parse(&config.mongodb_url).await?;
    let mongo_client = MongoClient::with_options(mongo_options)?;

    mongo_client
        .database("admin")
        .run_command(mongodb::bson::doc! { "ping": 1 })
        .await?;
    tracing::info!("MongoDB connecté");

    Ok(AppState {
        pg_pool,
        mongo_client,
    })
}
