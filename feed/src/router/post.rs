use axum::Router;
use axum::routing::{post};
use crate::handler::post::{create_post};

pub fn post_routers() -> Router {
    // 发布动态路由 需要验证权限 登录成功之后才能发布动态
    Router::new()
        .route("/posts", post( create_post ))
}