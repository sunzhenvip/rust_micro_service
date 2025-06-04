#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数
use axum::{Router, extract::Path, routing::get};

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
        .nest("/product", product_routers);

    // sleep(tokio::time::Duration::from_secs(5)).await;
    let host = "0.0.0.0:8080";
    // 绑定端口 启动服务
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn account() -> String {
    "account".to_string()
}

async fn product() -> String {
    "product".to_string()
}
