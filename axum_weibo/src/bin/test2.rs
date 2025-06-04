use axum::{Router, extract::Path, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/member", get(member))
        .route("/order/list", get(order_list))
        .route("/order/detail/:id", get(order_detail)) // 动态路由
        .route("/address/:city/:street", get(address))
        .route("/hello/*files", get(static_files));
    // sleep(tokio::time::Duration::from_secs(5)).await;
    let host = "0.0.0.0:8080";
    // 绑定端口 启动服务
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> String {
    "首页".to_string()
}

async fn member() -> String {
    "用户中心".to_string()
}

async fn order_list() -> String {
    "首页".to_string()
}

async fn order_detail(Path(order_id): Path<u64>) -> String {
    format!("订单详情 {}", order_id)
}

async fn address(Path((city, street)): Path<(String, String)>) -> String {
    format!("地址 {} {}", city , street)
}

//可以匹配 /hello/hhh /hello/hhhh/ghj/gdh 但是不能匹配/hello/
async fn static_files(Path(path): Path<String>) -> String {
    format!("path {}", path)
}
