use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberSession {
    pub member_id: Option<u64>,
    pub member_name: Option<String>,
}
