#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数
use axum::extract::Query;
use axum::headers::UserAgent;
use axum::http::HeaderMap;
use axum::routing::post;
use axum::{Json, Router, TypedHeader, routing::get};
use chrono::Local;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Write;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/movie/json", post(movie_json));

    // sleep(tokio::time::Duration::from_secs(5)).await;
    let host = "0.0.0.0:8080";
    // 绑定端口 启动服务
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 获取所有header内容
async fn movie_json(Json(data): Json<Value>) -> String {
    dbg!(data);
    format!(
        "打印时间 {} 执行的函数 {}",
        get_current_time(),
        "movie_json".to_string()
    )
}

fn get_current_time() -> String {
    let mut buf = String::with_capacity(19); // 预分配空间
    write!(&mut buf, "{}", Local::now().format("%Y-%m-%d %H:%M:%S")).unwrap();
    buf
}
