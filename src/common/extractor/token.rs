use crate::common::error::biz_error::BizError;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use std::ops::Deref;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct Token(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for Token
where
    S: Send + Sync,
{
    type Rejection = BizError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        debug!("started extracting token");
        let token = parts.headers.get("Token").and_then(|v| v.to_str().ok());
        debug!("token = {:?}", token);
        if let Some(token) = token {
            return Ok(Self(String::from(token)));
        }
        Err(BizError::TokenMissing)
    }
}

impl Deref for Token {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
