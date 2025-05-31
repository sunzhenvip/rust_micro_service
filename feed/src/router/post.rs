use axum::Router;
use axum::routing::{post};
use crate::handler::post::{create_post};

pub fn post_routers() -> Router {
    Router::new()
        .route("/posts", post( create_post ))
}