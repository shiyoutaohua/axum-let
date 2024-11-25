use crate::common::{
    constant::http::HEADER_CTX_ID, error::biz_error::BizError, util::id::UuidGenerator,
};
use axum::{middleware::Next, response::Response};
use tracing::trace;

pub async fn set_ctx_id(
    mut request: axum::extract::Request,
    next: Next,
) -> Result<Response, BizError> {
    trace!("start build request context");
    let uuid = UuidGenerator::next_v7();
    request
        .headers_mut()
        .insert(HEADER_CTX_ID, uuid.parse().unwrap());
    // call next service
    let response = next.run(request).await;
    // do something with `response`...
    trace!("finished build request context");
    Ok(response)
}
