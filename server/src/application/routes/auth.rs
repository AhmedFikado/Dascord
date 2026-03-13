use crate::application::controller::auth_controller::AuthHandler;
use crate::infrastructure::repositories::UserRepository;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

/// Configure les routes d'authentification
pub fn auth_routes<R: UserRepository + 'static>(handler: Arc<AuthHandler<R>>) -> Router {
    Router::new()
        .route("/signup", post(crate::application::controller::auth_controller::signup::<R>))
        .route("/login", post(crate::application::controller::auth_controller::login::<R>))
        .route("/logout", post(crate::application::controller::auth_controller::logout::<R>))
        .route("/me", get(crate::application::controller::auth_controller::get_me::<R>))
        .with_state(handler)
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::services::auth::{LoginUseCase, LogoutUseCase, SignupUseCase};
    use crate::infrastructure::repositories::mocks::mock_user_repository::MockUserRepository;
    use crate::infrastructure::security::JWTService;
    use crate::domain::services::user::UserService;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_auth_routes_creation() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());

        let signup_uc = SignupUseCase::new(user_service.clone(), jwt_service.clone());
        let login_uc = LoginUseCase::new(user_service.clone(), jwt_service.clone());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

        let handler = Arc::new(AuthHandler::new(
            signup_uc,
            login_uc,
            logout_uc,
            jwt_service,
        ));
        let app = auth_routes(handler);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/signup")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"username":"test","email":"test@test.com","password":"password123"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }
}
