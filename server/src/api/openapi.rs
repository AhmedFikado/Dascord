use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth
        crate::api::handlers::auth_handler::signup,
        crate::api::handlers::auth_handler::login,
        crate::api::handlers::auth_handler::logout,
        crate::api::handlers::auth_handler::get_me,

        // Users
        crate::api::handlers::user_handler::get_me,
        crate::api::handlers::user_handler::update_status,
        crate::api::handlers::user_handler::update_user,

        // Servers
        crate::api::handlers::server_handler::create_server,
        crate::api::handlers::server_handler::get_user_servers,
        crate::api::handlers::server_handler::get_server_info,
        crate::api::handlers::server_handler::update_server,
        crate::api::handlers::server_handler::delete_server,
        crate::api::handlers::server_handler::join_server,
        crate::api::handlers::server_handler::leave_server,
        crate::api::handlers::server_handler::list_members,
        crate::api::handlers::server_handler::update_member_role,
        crate::api::handlers::server_handler::get_channels,
        crate::api::handlers::server_handler::create_channel,

        // Channels
        crate::api::handlers::channel_handler::get_channel_info,
        crate::api::handlers::channel_handler::update_channel,
        crate::api::handlers::channel_handler::delete_channel,

        // Messages
        crate::api::handlers::message_handler::send_message,
        crate::api::handlers::message_handler::get_message_history,
        crate::api::handlers::message_handler::delete_message,
        crate::api::handlers::message_handler::update_message,
    ),
    components(
        schemas(
            crate::application::dto::auth::SignupRequest,
            crate::application::dto::auth::SignupResponse,
            crate::application::dto::auth::LoginRequest,
            crate::application::dto::auth::LoginResponse,
            crate::application::dto::auth::LogoutResponse,
            crate::application::dto::auth::UserResponse,
            crate::application::dto::server::CreateServerRequest,
            crate::application::dto::server::ServerResponse,
            crate::application::dto::server::JoinServerRequest,
            crate::application::dto::channel::CreateChannelRequest,
            crate::application::dto::channel::ChannelResponse,
            crate::application::dto::message_dto::MessageDto,
            crate::application::dto::message_dto::CreateMessageDto,
            crate::application::dto::user_dto::UserDto,
            crate::domain::value_objects::ServerRole,
        )
    ),
    tags(
        (name = "auth", description = "Endpoints d'authentification"),
        (name = "users", description = "Gestion des utilisateurs"),
        (name = "servers", description = "Gestion des serveurs"),
        (name = "channels", description = "Gestion des channels"),
        (name = "messages", description = "Gestion des messages"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
