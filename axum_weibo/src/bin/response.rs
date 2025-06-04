#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数
// 引入依赖包
use axum::body::Body;
use axum::body::{Bytes, HttpBody};
use axum::routing::{get};
use axum::{
    BoxError, Json, Router,
    extract::FromRequest,
    http::Request,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use validator::{Validate, ValidationError};

#[derive(Serialize)]
struct Car {
    brand: String,
    price: u32,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello_response", get(hello_response));

    // sleep(tokio::time::Duration::from_secs(5)).await;
    let host = "0.0.0.0:8080";
    // 绑定端口 启动服务
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_response() -> Json<Car> { // http 请求返回 json 数据
    let lx = Car {
        brand: "iPad Air 2025 128G".to_string(),
        price: 100,
    };
    Json(lx)
}
