use crate::repository::user_repository;
use serde::{Deserialize, Serialize};
use time::{macros::offset, OffsetDateTime};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserDetailResult {
    pub user_id: i64,
    pub nickname: String,
    pub password: String,
    pub email: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserEditResult {
    pub user_id: u64,
}

impl From<user_repository::Model> for UserDetailResult {
    fn from(value: user_repository::Model) -> Self {
        Self {
            user_id: value.user_id,
            nickname: value.nickname.clone(),
            password: value.password.clone(),
            email: value.email.clone(),
            created_at: Some(value.created_at.assume_offset(offset!(+8))),
            updated_at: Some(value.updated_at.assume_offset(offset!(+8))),
            deleted_at: value.deleted_at.map(|el| el.assume_offset(offset!(+8))),
        }
    }
}

impl From<&user_repository::Model> for UserDetailResult {
    fn from(value: &user_repository::Model) -> Self {
        Self {
            user_id: value.user_id,
            nickname: value.nickname.clone(),
            password: value.password.clone(),
            email: value.email.clone(),
            created_at: Some(value.created_at.assume_offset(offset!(+8))),
            updated_at: Some(value.updated_at.assume_offset(offset!(+8))),
            deleted_at: value.deleted_at.map(|el| el.assume_offset(offset!(+8))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserLoginResult {
    pub token: String,
}
