use crate::handler::member_handler;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new().nest(
        "/member",
        Router::new()
            .route("/count", get(member_handler::count))
            .route("/all", get(member_handler::all))
            .route("/exist", get(member_handler::exist_by_id))
            .route("/detail", get(member_handler::detail_by_id))
            .route("/delete", get(member_handler::delete_by_id))
            .route("/truncate", get(member_handler::truncate))
            .route("/login", post(member_handler::login)),
    )
}
