#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数

use axum::{Router};
use axum::http::{HeaderMap, HeaderValue, Method};
use axum::routing::{get, post};
// 中间件依赖包
use tower_http::trace::TraceLayer;
use tower_http::cors::{ CorsLayer};



#[tokio::main]
async fn main() {

    unsafe { // 这段代码为什么需要增加这个才能编译
        std::env::set_var("RUST_LOG", "tower_http=trace,middleware=trace");
    }
    // 初始化 日志  日志输出到屏幕上
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin("http://0.0.0.0:3000".parse::<HeaderValue>().unwrap());


    let app = Router::new().route("/hello_response", get(hello_response)).
        route("/hello_tower",get(hello_tower))
        .layer(TraceLayer::new_for_http()); // TraceLayer::new_for_http() 表示跟踪记录请求的

    // sleep(tokio::time::Duration::from_secs(5)).await;
    let host = "0.0.0.0:8080";
    // 绑定端口 启动服务
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn hello_response()->String{
    "hello_response".to_string()
}
async fn hello_tower()->String{
    "hello_tower".to_string()
}
