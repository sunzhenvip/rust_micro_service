#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数
// 引入依赖包
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
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
struct User {
    #[validate(length(min = 1), custom = "validate_username")]
    name: String,
    #[validate(range(
        min = 6,
        max = 26,
        message = "请输入正确范围 age length must be between 1 and 6"
    ))]
    age: u8,
    #[validate(email(message = "邮箱格式不正确"))] // 验证邮箱
    email: String,
    #[serde(rename = "my_address")] // 重命名
    address: Option<String>,
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    dbg!(username);
    if username == "admin" {
        return Err(ValidationError::new("参数错误"));
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello_user", post(hello_user));

    // sleep(tokio::time::Duration::from_secs(5)).await;
    let host = "0.0.0.0:8080";
    // 绑定端口 启动服务
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_user(Json(data): Json<User>) -> String {
    // 如果自己实现了 FromRequest  参数可以直接写 u: User 类似这样的
    // 获取请求信息
    dbg!(&data.name, data.age, &data.email, &data.address);
    if let Err(e) = data.validate() {
        return if e.field_errors().iter().len() > 1 {
            format!("msg validation error:\n{}", e) // 注意可能会返回多个
        } else {
            format!("msg validation error:{}", e)
        }
    }
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
