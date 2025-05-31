use axum::Router;
use axum::routing::{get};
use crate::handler::feed::get_feeds;

pub fn feed_routers() -> Router {
    Router::new()
        .route("/feeds", get( get_feeds ))
}