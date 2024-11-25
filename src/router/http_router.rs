use crate::handler::http_handler;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().nest("/http", Router::new().route("/ip", get(http_handler::ip)))
}
