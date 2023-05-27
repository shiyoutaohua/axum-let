pub mod common;
pub mod handler;
pub mod model;
pub mod repository;
pub mod router;

use crate::common::{context::app::AppContext, middleware::auth::member_auth, util::app};
use axum::{
    extract::DefaultBodyLimit,
    http::{HeaderName, Method},
    middleware, Extension, Router,
};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use router::{base_router, member_router};
use sqlx::mysql::MySqlPoolOptions;
use std::{net::SocketAddr, time::Duration};
use tokio_zookeeper::ZooKeeper;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, request_id::PropagateRequestIdLayer, trace::TraceLayer};
use tracing::{debug, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_thread_names(true)
        .init();

    // database
    let db_pool = MySqlPoolOptions::new()
        .max_connections(4)
        .min_connections(1)
        .connect("mysql://qingyuehanxi:08d5b53363b2051d@mysql.sqlpub.com:3306/poem1000")
        .await
        .expect("can't connect to database");
    // redis
    let redis_pool = Pool::builder()
        .max_size(4)
        .min_idle(Some(4))
        .build(RedisConnectionManager::new("redis://default:123456@10.0.0.10/").unwrap())
        .await
        .expect("can't connect to redis");
    // zookeeper
    let (zk, _watcher) = ZooKeeper::connect(&"10.0.0.10:2181".parse().unwrap())
        .await
        .expect("can't connect to zookeeper");
    // application context
    let app_ctx = AppContext {
        app_name: "axum-let",
        app_version: "1.0.0",
        author: "moon",
    };

    let cors_layer = CorsLayer::new()
        .allow_origin([
            "http://localhost".parse().unwrap(),
            "http://localhost:8080".parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3000));

    let router = Router::new()
        .merge(base_router::routes())
        .merge(member_router::routes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(DefaultBodyLimit::max(250 * 1024 * 1024))
                .layer(PropagateRequestIdLayer::new(HeaderName::from_static(
                    "x-request-id",
                )))
                .layer(cors_layer)
                .layer(Extension(app_ctx))
                .layer(Extension(zk))
                .layer(Extension(db_pool))
                .layer(Extension(redis_pool))
                .layer(middleware::from_fn(member_auth)),
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    debug!("listening on {}", addr);

    let _ = axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(app::shutdown())
        .await;
}
