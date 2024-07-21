pub struct Constants {}

impl Constants {
    pub const WIN_RUNTIME_PATH: &'static str = "C:/ProgramData/Vanguard";
    pub const LINUX_RUNTIME_PATH: &'static str = "/etc/vanguard";
    pub const OSX_RUNTIME_PATH: &'static str = "Library/Application Support/Vanguard";

    pub const SETTINGS_FILENAME: &'static str = "settings.json";
    pub const SESSION_FILENAME: &'static str = ".session.json";
    pub const ROUTER_FILENAME: &'static str = "routing.json";

    pub const DEFAULT_HTTP_IP: &'static str = "0.0.0.0";
    pub const DEFAULT_HTTP_PORT: u16 = 80;

    pub const DEFAULT_HTTPS_IP: &'static str = "0.0.0.0";
    pub const DEFAULT_HTTPS_PORT: u16 = 443;

    pub const DEFAULT_RPC_IP: &'static str = "127.0.0.1";
    pub const DEFAULT_RPC_PORT: u16 = 4242;
    pub const DEFAULT_PRIVATE_KEY: &'static str = "0xVanguard";
}
