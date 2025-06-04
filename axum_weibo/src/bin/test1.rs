#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    // 网页请求访问 http://localhost:3000/home
    let app:Router = Router::new().route("/home", get(home));

    let host = "0.0.0.0:3000";
    // 绑定端口 启动服务
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> String {
    "Hello, world!".to_string()
}