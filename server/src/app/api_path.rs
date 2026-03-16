pub struct ApiPath;

impl ApiPath {
    // Auth routes
    pub const AUTH_SIGNUP: &'static str = "/auth/signup";
    pub const AUTH_LOGIN: &'static str = "/auth/login";
    pub const AUTH_LOGOUT: &'static str = "/auth/logout";
    pub const AUTH_ME: &'static str = "/auth/me";

    // Users routes
    pub const USERS_ME: &'static str = "/users/me";
    pub const USERS_ME_STATUS: &'static str = "/users/me/status";

    // Servers routes
    pub const SERVERS: &'static str = "/servers";
    pub const SERVERS_ID: &'static str = "/servers/:id";
    pub const SERVERS_JOIN: &'static str = "/servers/join";
    pub const SERVERS_LEAVE: &'static str = "/servers/:id/leave";
    pub const SERVERS_MEMBERS: &'static str = "/servers/:id/members";
    pub const SERVERS_MEMBER: &'static str = "/servers/:id/members/:userId";
    pub const SERVERS_CHANNELS: &'static str = "/servers/:id/channels";

    // Channels routes
    pub const CHANNELS_ID: &'static str = "/channels/:id";
    pub const CHANNELS_MESSAGES: &'static str = "/channels/:id/messages";

    // Messages routes
    pub const MESSAGES_ID: &'static str = "/messages/:id";

    // WebSocket
    pub const WS: &'static str = "/ws";
}
