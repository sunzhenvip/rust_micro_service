#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数
use axum::extract::ConnectInfo;
use axum::extract::Extension;
use axum::http::{StatusCode, Uri};
use axum::{Router, extract::Path, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let account_routers = Router::new()
        .route("/info", get(account))
        .route("/reset", get(account));

    let product_routers = Router::new()
        .route("/detail", get(product))
        .route("/shop", get(product));

    let app = Router::new()
        .nest("/account", account_routers)
        .nest("/product", product_routers)
        .fallback(fail);

    // sleep(tokio::time::Duration::from_secs(5)).await;
    let host = "0.0.0.0:8080";
    // 绑定端口 启动服务
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn fail(uri: Uri) -> (StatusCode, String) {
    let host = uri.host().unwrap_or("unknown_host");
    // let host = uri.host()
    //     .or(uri.authority().map(|a| a.host()))
    //     .unwrap_or("unknown_host");
    (
        StatusCode::NOT_FOUND,
        format!(
            "您访问的文件不存在 {} {} {}",
            host,
            StatusCode::NOT_FOUND,
            uri
        ),
    )
}

async fn fail1(uri: Uri, Extension(addr): Extension<Option<SocketAddr>>) -> (StatusCode, String) {
    let client_addr = addr.map(|a| a.to_string()).unwrap_or("unknown".into());
    (
        StatusCode::NOT_FOUND,
        format!("Client: {}, URI: {}", client_addr, uri),
    )
}

async fn account() -> String {
    "account".to_string()
}

async fn product() -> String {
    "product".to_string()
}
