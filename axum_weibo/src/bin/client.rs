#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数

use axum_weibo::helloworld::greeter_client::GreeterClient;
use axum_weibo::helloworld::HelloRequest;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;

    let req = Request::new(HelloRequest {
        name: "张三".to_owned(),
    });

    let res = client.say_hello(req).await?;
    // 格式化输出
    println!("RESPONSE={:#?}", res);
    Ok(())
}
