use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth
        crate::application::controller::auth_controller::signup,
        crate::application::controller::auth_controller::login,
        crate::application::controller::auth_controller::logout,
        crate::application::controller::auth_controller::get_me,

        // Users
        crate::application::controller::user_controller::get_me,
        crate::application::controller::user_controller::update_status,
        crate::application::controller::user_controller::update_user,

        // Servers
        crate::application::controller::server_controller::create_server,
        crate::application::controller::server_controller::get_user_servers,
        crate::application::controller::server_controller::get_server_info,
        crate::application::controller::server_controller::update_server,
        crate::application::controller::server_controller::delete_server,
        crate::application::controller::server_controller::join_server,
        crate::application::controller::server_controller::leave_server,
        crate::application::controller::server_controller::list_members,
        crate::application::controller::server_controller::update_member_role,
        crate::application::controller::server_controller::get_channels,
        crate::application::controller::server_controller::create_channel,

        // Channels
        crate::application::controller::channel::channel_controller::get_channel_info,
        crate::application::controller::channel::channel_controller::update_channel,
        crate::application::controller::channel::channel_controller::delete_channel,
        crate::application::controller::channel::private_channel_controller::create_private_channel,
        crate::application::controller::channel::private_channel_controller::get_list_private_channels,
        crate::application::controller::channel::private_channel_controller::get_private_channel,

        // Messages
        crate::application::controller::message::message_controller::send_message,
        crate::application::controller::message::message_controller::get_message_history,
        crate::application::controller::message::message_controller::delete_message,
        crate::application::controller::message::message_controller::update_message,
        crate::application::controller::message::message_controller::send_welcome_message,
        crate::application::controller::message::private_message_controller::get_private_message_history,
        crate::application::controller::message::private_message_controller::send_private_message,
        crate::application::controller::message::private_message_controller::delete_private_message,
        crate::application::controller::message::private_message_controller::update_private_message,
    ),
    components(
        schemas(
            crate::application::dto::auth::SignupRequest,
            crate::application::dto::auth::SignupResponse,
            crate::application::dto::auth::LoginRequest,
            crate::application::dto::auth::LoginResponse,
            crate::application::dto::auth::LogoutResponse,
            crate::application::dto::auth::UserResponse,
            crate::application::dto::server_dto::CreateServerRequest,
            crate::application::dto::server_dto::ServerResponse,
            crate::application::dto::server_dto::JoinServerRequest,
            crate::application::dto::channel::CreateChannelRequest,
            crate::application::dto::channel::ChannelResponse,
            crate::application::dto::channel::CreatePrivateChannelRequest,
            crate::application::dto::channel::PrivateChannelResponse,
            crate::application::dto::message::MessageDto,
            crate::application::dto::message::CreateMessageDto,
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
