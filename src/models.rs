use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub ready: bool,
    pub live: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            ready: true,
            live: true,
        }
    }
}
