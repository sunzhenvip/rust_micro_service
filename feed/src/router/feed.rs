use axum::Router;
use axum::routing::{get};
use crate::handler::feed::get_feeds;

pub fn feed_routers() -> Router {
    // 获取信息流
    Router::new()
        .route("/feeds", get( get_feeds ))
}