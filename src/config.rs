use std::env;

pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: &str = "3000";

pub const READYZ_ROUTE: &str = "/readyz";
pub const LIVEZ_ROUTE: &str = "/livez";

pub fn get_server_addr() -> String {
    let host = env::var("HTTP_HOST").unwrap_or_else(|_| DEFAULT_HOST.into());
    let port = env::var("HTTP_PORT").unwrap_or_else(|_| DEFAULT_PORT.into());
    format!("{host}:{port}")
}
