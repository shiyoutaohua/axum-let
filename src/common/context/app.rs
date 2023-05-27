use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct AppContext {
    pub app_name: &'static str,
    pub app_version: &'static str,
    pub author: &'static str,
}
