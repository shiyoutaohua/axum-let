use axum::{response::IntoResponse, Extension};
use reqwest::Client;

use crate::model::result::base::BizResult;

pub async fn ip(Extension(client): Extension<Client>) -> impl IntoResponse {
    let ip = client
        .get("https://httpbin.org/ip")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    BizResult::ok(ip)
}
