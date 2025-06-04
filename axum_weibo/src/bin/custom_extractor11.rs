#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数

use axum::body::Body;
use axum::body::{Bytes, HttpBody};
use axum::routing::post;
use axum::{
    BoxError, Json, RequestExt, Router, async_trait,
    extract::FromRequest,
    http::Request,
    response::{IntoResponse, Response},
};
use chrono::Local;
use serde::Deserialize;
use std::fmt::Write;

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: u8,
    address: Option<String>,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for User
where
    B: HttpBody + Send + 'static,
    B::Data: Into<Bytes> + Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = Response; // 返回一个错误的响应信息

    // 占时用不到 _state 这个变量 可以先不管他
    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = req
            .extract::<Json<User>, _>()
            .await
            .map_err(|err| err.into_response())?;
        Ok(payload)
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello_user", post(hello_user));

    // sleep(tokio::time::Duration::from_secs(5)).await;
    let host = "0.0.0.0:8082";
    // 绑定端口 启动服务
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_user(u: User) -> String {
    // 如果自己实现了 FromRequest  参数可以直接写 u: User 类似这样的
    // 获取请求信息
    dbg!(u.name, u.age, u.address);
    format!(
        "打印时间 {} 执行的函数 {}",
        get_current_time(),
        "hello_user".to_string()
    )
}

fn get_current_time() -> String {
    let mut buf = String::with_capacity(19); // 预分配空间
    write!(&mut buf, "{}", Local::now().format("%Y-%m-%d %H:%M:%S")).unwrap();
    buf
}
