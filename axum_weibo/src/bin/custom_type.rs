#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数
use axum::body::Body;
use axum::http::Request;
use axum::routing::post;
use axum::{Json, Router};
use chrono::Local;
use serde::Deserialize;
use std::fmt::Write;

// 自定义类型
// 使用 serde里面的反序列化派生宏
#[derive(Deserialize)]
struct User {
    name: String,
    age: u8,
    address: Option<String>, // 增加了 Option 提交的数据 可以没有这个字段
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/user/json", post(user_json));

    // sleep(tokio::time::Duration::from_secs(5)).await;
    let host = "0.0.0.0:8082";
    // 绑定端口 启动服务
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn user_json(Json(data): Json<User>) -> String {
    // 获取请求信息
    dbg!(data.name,data.age,data.address);
    format!(
        "打印时间 {} 执行的函数 {}",
        get_current_time(),
        "user_json".to_string()
    )
}

fn get_current_time() -> String {
    let mut buf = String::with_capacity(19); // 预分配空间
    write!(&mut buf, "{}", Local::now().format("%Y-%m-%d %H:%M:%S")).unwrap();
    buf
}
