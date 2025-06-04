#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数

use axum::body::{Body, HttpBody};
use axum::http::{Request, request};
use axum::routing::post;
use axum::{Json, Router};
use chrono::Local;
use serde_json::Value;
use std::fmt::Write;
#[tokio::main]
async fn main() {
    let app = Router::new().route("/movie/request", post(request));

    // sleep(tokio::time::Duration::from_secs(5)).await;
    let host = "0.0.0.0:8082";
    // 绑定端口 启动服务
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn request(request: Request<Body>) -> String {
    // 获取请求信息
    dbg!(request.uri());
    dbg!(request.headers());
    dbg!(request.into_body().data().await);
    format!(
        "打印时间 {} 执行的函数 {}",
        get_current_time(),
        "request".to_string()
    )
}

fn get_current_time() -> String {
    let mut buf = String::with_capacity(19); // 预分配空间
    write!(&mut buf, "{}", Local::now().format("%Y-%m-%d %H:%M:%S")).unwrap();
    buf
}
