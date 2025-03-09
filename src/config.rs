use std::env;

pub const READYZ_ROUTE: &str = "/readyz";
pub const LIVEZ_ROUTE: &str = "/livez";

pub fn get_server_addr() -> String {
    let host = env::var("HTTP_HOST").unwrap_or("127.0.0.1".into());
    let port = env::var("HTTP_PORT").unwrap_or("3000".into());
    format!("{host}:{port}")
}
