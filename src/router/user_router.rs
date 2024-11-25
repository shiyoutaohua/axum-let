use crate::handler::user_handler;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new().nest(
        "/user",
        Router::new()
            .route("/all", get(user_handler::all))
            .route("/list-by-id", post(user_handler::list_by_id))
            .route("/detail-by-id", get(user_handler::detail_by_id))
            .route("/login", post(user_handler::login)),
    )
}
