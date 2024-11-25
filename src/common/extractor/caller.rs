use crate::common::{error::biz_error::BizError, extractor::apikey::Apikey};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Caller {
    pub apikey: String,
    pub appname: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Caller
where
    S: Send + Sync,
{
    type Rejection = BizError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        debug!("start extracting caller");
        let apikey = parts.extract::<Apikey>().await?;
        let apikey = (*apikey).to_string();
        if apikey.eq("qyhx") {
            Ok(Self {
                apikey: apikey.clone(),
                appname: format!("{}:{}", "appname", apikey.clone()),
            })
        } else {
            Err(BizError::APIKEY_INVALID)
        }
    }
}
