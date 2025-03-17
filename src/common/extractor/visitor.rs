use std::ops::Deref;

use crate::{
    common::{error::biz_error::BizError, extractor::token::Token},
    model::dto::user::UserSession,
};
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Visitor {
    pub user_id: i64,
}

impl<S> FromRequestParts<S> for Visitor
where
    S: Send + Sync,
{
    type Rejection = BizError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        debug!("start extracting visitor");
        let token = parts.extract::<Token>().await?;
        let redis_pool = parts
            .extensions
            .get_mut::<Pool<RedisConnectionManager>>()
            .unwrap();
        let mut conn = redis_pool.get().await.unwrap();

        let reply: Result<String, redis::RedisError> = conn.get((*token).clone()).await;
        match reply {
            Ok(reply) => {
                let user_session: UserSession = serde_json::from_str(&reply).unwrap();
                debug!("find session = {:?}", user_session);
                Ok(Self {
                    user_id: user_session.user_id,
                })
            }
            Err(error) => {
                debug!("find session error = {:?}", error);
                return Err(BizError::TOKEN_INVALID);
            }
        }
    }
}

impl Deref for Visitor {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.user_id
    }
}
