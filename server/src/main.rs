use std::sync::Arc;
use server::{
    config::{AppConfig, DatabaseConfig},
    infrastructure::database::init_databases,
    infrastructure::security::JWTService,
    infrastructure::websocket::{ConnectionManager, WebSocketState, create_ws_router},
    application::use_cases::auth::{SignupUseCase, LoginUseCase, LogoutUseCase},
    mocks::MockUserService,
    api::router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    dotenvy::dotenv().ok();

    let app_config = AppConfig::from_env();
    let db_config = DatabaseConfig::from_env();

    tracing::info!("Configuration chargée");
    tracing::info!("   - Serveur: {}", app_config.address());
    tracing::info!("   - PostgreSQL: {}", db_config.postgres_url);
    tracing::info!("   - MongoDB: {}", db_config.mongodb_url);

    let _app_state = init_databases(&db_config).await?;
    tracing::info!("Connexions aux bases de données établies");

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev_secret_key_change_in_production".to_string());
    let jwt_service = JWTService::new(jwt_secret);

    // TODO: Remplacer MockUserService par PostgresUserRepository
    let user_service = MockUserService::new();
    
    let signup_uc = SignupUseCase::new(user_service.clone(), jwt_service.clone());
    let login_uc = LoginUseCase::new(user_service.clone(), jwt_service.clone());
    let logout_uc = LogoutUseCase::new(user_service.clone(), jwt_service.clone());

    // Créer le gestionnaire WebSocket
    let ws_manager = Arc::new(ConnectionManager::new());
    let ws_state = WebSocketState {
        manager: ws_manager,
        jwt_service: jwt_service.clone(),
    };

    // Créer les routes HTTP et WebSocket
    let http_router = router::create_router(signup_uc, login_uc, logout_uc, jwt_service, user_service);
    let ws_router = create_ws_router(ws_state);
    
    // Fusionner les deux routers
    let app = http_router.merge(ws_router);

    let listener = tokio::net::TcpListener::bind(&app_config.address())
        .await?;

    tracing::info!("Serveur démarré sur {}", app_config.address());
    tracing::info!("Endpoints HTTP:");
    tracing::info!("   POST /auth/signup");
    tracing::info!("   POST /auth/login");
    tracing::info!("   POST /auth/logout");
    tracing::info!("   GET  /auth/me");
    tracing::info!("Endpoints WebSocket:");
    tracing::info!("   WS   /ws?token=<jwt_token>");
    tracing::info!("   GET  /users/me");

    axum::serve(listener, app).await?;

    Ok(())
}
