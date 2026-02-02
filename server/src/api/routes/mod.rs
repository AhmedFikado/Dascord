pub mod auth;
pub mod channels;
pub mod servers;
pub mod messages;
pub mod user_routes;

pub use auth::auth_routes;
pub use channels::channel_routes;
pub use servers::server_routes;
pub use messages::message_routes;
pub use user_routes::user_routes;