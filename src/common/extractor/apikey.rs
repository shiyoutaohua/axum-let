use crate::common::{constant::http::HEADER_APIKEY, error::biz_error::BizError};
use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use tracing::debug;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Apikey(pub String);

impl<S> FromRequestParts<S> for Apikey
where
    S: Send + Sync,
{
    type Rejection = BizError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        debug!("start extracting apikey");
        let apikey = parts
            .headers
            .get(HEADER_APIKEY)
            .and_then(|v| v.to_str().ok());
        if let Some(apikey) = apikey {
            debug!("apikey = {:?}", apikey);
            Ok(Self(String::from(apikey)))
        } else {
            debug!("apikey missing");
            Err(BizError::APIKEY_MISSING)
        }
    }
}

impl Deref for Apikey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
