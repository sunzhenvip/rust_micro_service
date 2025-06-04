use axum::Router;
use axum::headers::HeaderMap;
use axum::http::Method;
use axum::routing::post;
use chrono::Local;
use std::fmt::Write;
// 10.参数顺序函数按从左到右的顺序提取参数，request body是一个只能被使用一次的异步流，所以有多个参数时它是最后一个参数

#[tokio::main]
async fn main() {
    let app = Router::new().route("/data", post(data_fn));

    // sleep(tokio::time::Duration::from_secs(5)).await;
    let host = "0.0.0.0:8082";
    // 绑定端口 启动服务
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// body位置不能互换 body 只能放到参数最后一个位置
async fn data_fn(method: Method, headers: HeaderMap, body: String) -> String {
    // 获取请求信息
    let request_info = format!("{:?} {:?} {:?}", method, headers, body);
    let time_info = format!(
        "打印时间 {} 执行的函数 {}",
        get_current_time(),
        "user_json".to_string()
    );
    format!("{}\n{}", request_info, time_info)
}

fn get_current_time() -> String {
    let mut buf = String::with_capacity(19); // 预分配空间
    write!(&mut buf, "{}", Local::now().format("%Y-%m-%d %H:%M:%S")).unwrap();
    buf
}
