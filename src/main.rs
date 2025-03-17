use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use axum::{extract::DefaultBodyLimit, middleware, Extension, Router};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use common::{middleware::request_context::set_ctx_id, util::app};
use router::{base_router, http_router, user_router};
use sea_orm::{ConnectOptions, Database};
use tokio_zookeeper::ZooKeeper;
use tower::ServiceBuilder;
use tower_http::{
    cors::{AllowCredentials, AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    request_id::PropagateRequestIdLayer,
    services::ServeDir,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::{debug, Level};

pub mod common;
pub mod handler;
pub mod model;
pub mod repository;
pub mod router;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_thread_names(true)
        .with_timer(
            tracing_subscriber::fmt::time::OffsetTime::local_rfc_3339()
                .expect("can't get local offset"),
        )
        .init();

    common::cfg::app::configure();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn(|| {
            static TOKIO_WORKER_ID: AtomicU64 = AtomicU64::new(0);
            let id = TOKIO_WORKER_ID.fetch_add(1, Ordering::SeqCst);
            format!("tokio-worker-{id}")
        })
        .build()
        .expect("can't create tokio runtime");
    rt.block_on(init());
}

pub async fn init() {
    // cors
    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .allow_methods(AllowMethods::mirror_request())
        .allow_headers(AllowHeaders::mirror_request())
        .allow_credentials(AllowCredentials::yes())
        .max_age(Duration::from_secs(3000));
    // http client
    let http_client = reqwest::Client::new();
    // zookeeper
    let (zk, _watcher) = ZooKeeper::connect(&"121.37.129.161:2181".parse().unwrap())
        .await
        .expect("can't connect to zookeeper");
    // redis
    let redis_pool = Pool::builder()
        .max_size(4)
        .min_idle(Some(4))
        .build(
            RedisConnectionManager::new("redis://default:China@1949@121.37.129.161:6379").unwrap(),
        )
        .await
        .expect("can't connect to redis");
    // datasource
    let ds_opt =
        ConnectOptions::new("postgres://postgres:70617373776f7264@121.37.129.161:5432/db1")
            .max_connections(4)
            .min_connections(1)
            .sqlx_logging(false)
            .to_owned();
    let ds = Database::connect(ds_opt)
        .await
        .expect("can't connect to database");
    // router
    let app_router = Router::new()
        .merge(base_router::routes())
        .merge(http_router::routes())
        .merge(user_router::routes())
        .fallback(app::handler_404)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(Duration::from_secs(3)))
                .layer(DefaultBodyLimit::max(250 * 1024 * 1024))
                .layer(middleware::from_fn(set_ctx_id))
                .layer(PropagateRequestIdLayer::x_request_id())
                .layer(cors_layer)
                .layer(Extension(http_client))
                .layer(Extension(zk))
                .layer(Extension(redis_pool))
                .layer(Extension(ds)),
        );
    // curl http://localhost/res/application.toml
    let res_router = Router::new()
        .nest_service("/res", ServeDir::new("res"))
        .fallback(app::handler_404);

    let app_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 8080);
    let res_addr = SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 8081);
    debug!("app listening on {}", app_addr);
    debug!("res listening on {}", res_addr);
    tokio::join!(
        app::serve(app_router, app_addr),
        app::serve(res_router, res_addr),
    );
}
