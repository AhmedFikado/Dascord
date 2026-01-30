use axum::{routing::post, Router};
use server::{
    config::{AppConfig, DatabaseConfig},
    infrastructure::database::init_databases,
    infrastructure::security::{JWTService, PasswordService},
    application::use_cases::auth::{SignupUseCase, LoginUseCase, LogoutUseCase},
    mocks::{MockUserService, mock_auth_handler::AuthHandler},
};
use std::sync::Arc;

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

    tracing::info!("🔧 Configuration chargée");
    tracing::info!("   - Serveur: {}", app_config.address());
    tracing::info!("   - PostgreSQL: {}", db_config.postgres_url);
    tracing::info!("   - MongoDB: {}", db_config.mongodb_url);

    // Initialisation des bases de données
    let app_state = init_databases(&db_config).await?;
    tracing::info!("Connexions aux bases de données établies");

    // Configuration des services
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev_secret_key_change_in_production".to_string());
    
    let jwt_service = JWTService::new(jwt_secret);
    let password_service = PasswordService::new();
    // Remplacer MockUserService par un vrai UserService utilisant un user repository lorsqu'il sera dispo
    let user_service = MockUserService::new();
    
    // Use cases
    let signup_uc = SignupUseCase::new(user_service.clone(), jwt_service.clone());
    let login_uc = LoginUseCase::new(user_service.clone(), jwt_service.clone());
    let logout_uc = LogoutUseCase::new(user_service, jwt_service);
    
    let auth_handler = Arc::new(AuthHandler::new(signup_uc, login_uc, logout_uc));

    // Configuration des routes
    let app = Router::new()
        .route("/auth/signup", post(AuthHandler::signup))
        .route("/auth/login", post(AuthHandler::login))
        .route("/auth/logout", post(AuthHandler::logout))
        .with_state(auth_handler);

    // Démarrage du serveur
    let listener = tokio::net::TcpListener::bind(&app_config.address())
        .await?;

    tracing::info!("Serveur démarré sur {}", app_config.address());
    tracing::info!("Endpoints disponibles:");
    tracing::info!("   POST /auth/signup");
    tracing::info!("   POST /auth/login");
    tracing::info!("   POST /auth/logout");

    axum::serve(listener, app).await?;

    Ok(())
}
