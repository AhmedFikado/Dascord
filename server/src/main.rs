use server::{
    config::{AppConfig, DatabaseConfig},
    domain::entities::Message,
    infrastructure::database::init_databases,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration du logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    // Charger les variables d'environnement
    dotenvy::dotenv().ok();

    // Configuration
    let app_config = AppConfig::from_env();
    let db_config = DatabaseConfig::from_env();

    tracing::info!("Configuration chargée");
    tracing::info!("   - Serveur: {}", app_config.address());
    tracing::info!("   - PostgreSQL: {}", db_config.postgres_url);
    tracing::info!("   - MongoDB: {}", db_config.mongodb_url);

    // Initialisation des bases de données
    let app_state = init_databases(&db_config).await?;

    tracing::info!("Connexions établies avec succès");
    tracing::info!("Entities disponibles: User, Server, Channel, Message");
    tracing::info!("DTOs disponibles: UserDto, MessageDto");
    
    // Test simple de connexion PostgreSQL
    let user_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(&app_state.pg_pool)
        .await?;
    tracing::info!("Nombre d'utilisateurs dans PostgreSQL: {}", user_count);

    // Test simple de connexion MongoDB
    let db = app_state.mongo_client.database("rtc_db");
    let message_count = db.collection::<Message>("messages")
        .count_documents(mongodb::bson::doc! {})
        .await?;
    tracing::info!("Nombre de messages dans MongoDB: {}", message_count);

    tracing::info!("Tout fonctionne ! Prêt pour développer les handlers.");

    Ok(())
}
