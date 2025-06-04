use axum::Router;
use axum::routing::{delete, post};
use crate::handler::post::{create_post, delete_post};

pub fn post_routers() -> Router {
    Router::new()
        .route("/posts", post( create_post ))
        .route("/posts/:pid", delete(delete_post))
}