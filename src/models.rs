use serde::{Deserialize, Serialize};

pub const DEFAULT_READY: bool = true;
pub const DEFAULT_LIVE: bool = true;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub ready: bool,
    pub live: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            ready: DEFAULT_READY,
            live: DEFAULT_LIVE,
        }
    }
}
