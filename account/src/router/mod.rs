pub mod user;

use axum::{middleware, Router};
use crate::middleware::limiter::Limiter;
use crate::router::user::user_routers;

pub async fn start_route() {
    //用户路由
    let user_routers = user_routers();

    let app = Router::new()
        .merge(user_routers)
        .layer(middleware::from_extractor::<Limiter>());

    println!("服务器启动了");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}