use crate::api::handlers::user_handler::{get_me, update_status, update_user};
use crate::api::handlers::UserHandler;
use crate::infrastructure::repositories::{ServerRepository, UserRepository};
use axum::{routing::{get, put}, Router};
use std::sync::Arc;

/// Routes pour les utilisateurs
pub fn user_routes<R: UserRepository + 'static, SR: ServerRepository + 'static>(handler: Arc<UserHandler<R, SR>>) -> Router {
    Router::new()
        .route("/me", get(get_me::<R>))
        .route("/me/status", put(update_status::<R>))
        .route("/update_user", put(update_user::<R>))
        .with_state(handler)
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::mocks::mock_server_repository::MockServerRepository;
    use crate::infrastructure::repositories::mocks::mock_user_repository::MockUserRepository;
    use crate::infrastructure::security::JWTService;
    use crate::infrastructure::services::UserService;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    type TestHandler = UserHandler<MockUserRepository, MockServerRepository>;

    #[tokio::test]
    async fn test_user_routes_get_me_unauthorized() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(TestHandler::new(user_service, jwt_service));
        let app = user_routes(handler);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/me")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_user_routes_update_user_unauthorized() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(TestHandler::new(user_service, jwt_service));
        let app = user_routes(handler);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/update_user")
                    .method("PUT")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"username":"x","email":"x@x.com"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
