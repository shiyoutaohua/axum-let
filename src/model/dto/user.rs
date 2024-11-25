use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: i64,
    pub nickname: Option<String>,
}
