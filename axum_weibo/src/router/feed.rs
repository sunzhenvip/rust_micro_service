use axum::Router;
use axum::routing::{post};
use crate::handler::feed::feed;


pub fn feed_routers() -> Router {
    Router::new()
        .route("/feeds", post( feed ))
}