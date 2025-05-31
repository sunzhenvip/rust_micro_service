pub mod follower;

use axum::{middleware, Router};
use crate::middleware::jwt::Auth;
use crate::router::feed::feed_routers;
use crate::router::follower::follower_routers;
use crate::router::post::post_routers;
use crate::router::user::user_routers;

pub async fn start_route() {
    //用户路由
    let follower_routers = follower_routers();

    let app = Router::new()
        .merge(follower_routers)
        .layer(middleware::from_extractor::<Auth>());

    axum::Server::bind(&"0.0.0.0:8082".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}