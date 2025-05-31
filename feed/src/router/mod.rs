pub mod post;
pub mod feed;

use axum::{middleware, Router};
use crate::middleware::jwt::Auth;
use crate::router::feed::feed_routers;
use crate::router::post::post_routers;

pub async fn start_route() {
    //用户路由
    let post_routers = post_routers();
    let feed_routers = feed_routers();

    let app = Router::new()
        .merge(post_routers)
        .merge(feed_routers)
        .layer(middleware::from_extractor::<Auth>());

    axum::Server::bind(&"0.0.0.0:8083".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}