#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)]

use std::collections::HashMap;
// 消除 未使用的变量/参数
use axum::extract::Query;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/move", get(movie));

    // sleep(tokio::time::Duration::from_secs(5)).await;
    let host = "0.0.0.0:8080";
    // 绑定端口 启动服务
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn movie(Query(query): Query<HashMap<String, String>>) -> String {
    // dbg!(query);
    dbg!(query.get("a"));
    "move success".to_string()
}
