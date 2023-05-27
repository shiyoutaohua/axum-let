use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberLoginParam {
    pub email: String,
    pub login_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberExistParam {
    pub member_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberDetailParam {
    pub member_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberDeleteParam {
    pub member_id: Option<u64>,
}
