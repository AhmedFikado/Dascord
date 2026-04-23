pub mod auth_routes;
pub mod channels_routes;
pub mod messages_routes;
pub mod servers_routes;
pub mod user_routes;
pub mod avatar_routes;

pub use auth_routes::auth_routes;
pub use channels_routes::channel_routes;
pub use messages_routes::message_routes;
pub use servers_routes::server_routes;
pub use user_routes::user_routes;
pub use avatar_routes::avatar_routes;
