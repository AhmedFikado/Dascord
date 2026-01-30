pub mod routes;
pub mod handlers;
pub mod router;

pub use handlers::AuthHandler;
pub use routes::auth_routes;
pub use router::create_router;