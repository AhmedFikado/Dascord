pub mod auth;
pub mod channels;
pub mod servers;
pub mod messages;

pub use auth::auth_routes;
pub use channels::channel_routes;
pub use servers::server_routes;
pub use messages::message_routes;