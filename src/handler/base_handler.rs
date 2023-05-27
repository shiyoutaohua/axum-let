use crate::{common::context::app::AppContext, model::result::base::BizResult};
use axum::{
    extract::{Multipart, Path as PathVar, Query},
    headers::UserAgent,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json, TypedHeader,
};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use std::{collections::HashMap, path::Path};
use tokio::{
    fs::OpenOptions,
    io::{AsyncWriteExt, BufWriter},
};
use tokio_zookeeper::ZooKeeper;

pub async fn greet(Extension(app_ctx): Extension<AppContext>) -> impl IntoResponse {
    let reply = format!("Hey - {}", app_ctx.app_name);
    Json(BizResult::ok(reply))
}

pub async fn header(TypedHeader(user_agent): TypedHeader<UserAgent>) -> impl IntoResponse {
    Json(BizResult::ok(format!("{:?}", user_agent)))
}

pub async fn headers(header_map: HeaderMap) -> impl IntoResponse {
    Json(BizResult::ok(format!("{:?}", header_map)))
}

pub async fn path(PathVar(key): PathVar<String>) -> impl IntoResponse {
    String::from(key)
}

pub async fn query(Query(map): Query<HashMap<String, String>>) -> impl IntoResponse {
    format!("{:?}", map)
}

pub async fn post_text(body: String) -> impl IntoResponse {
    Json(BizResult::ok(body))
}

pub async fn download_file() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment;filename=Cargo.toml".parse().unwrap(),
    );
    let data = tokio::fs::read(Path::new("Cargo.toml")).await.unwrap();
    (headers, data)
}

#[allow(unused_must_use)]
pub async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        if let Some(origin_file_name) = field.file_name() {
            let dst = std::env::current_dir()
                .unwrap()
                .join("target")
                .join(origin_file_name);
            let dst = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(dst)
                .await
                .unwrap();
            let mut writer = BufWriter::new(dst);
            while let Some(data) = field.chunk().await.unwrap() {
                writer.write_all(&*data).await;
            }
            writer.flush().await;
        }
    }
    Json(BizResult::ok(()))
}

pub async fn ping_redis(
    Extension(redis_pool): Extension<Pool<RedisConnectionManager>>,
) -> impl IntoResponse {
    let mut conn = redis_pool.get().await.unwrap();
    let reply: String = redis::cmd("PING").query_async(&mut *conn).await.unwrap();
    Json(BizResult::ok(format!("{}", reply)))
}

pub async fn ping_zk(Extension(zk): Extension<ZooKeeper>) -> impl IntoResponse {
    let state = zk.watch().exists("/zookeeper").await.unwrap();
    Json(BizResult::ok(format!("{:?}", state)))
}

/// 404 handler
pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "How ! 404.")
}
