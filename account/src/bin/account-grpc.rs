use anyhow::anyhow;
use etcd_client::Client;
use opentelemetry::global;
use tonic::{transport::Server};
use account::config::config_init;
use account::handler::user_rpc::AccountRpc;
use account::model::db_conn_init;
use account::proto::account::account_server::{AccountServer};
use tools::service::register::Register;
use tools::trace::init_tracer;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    config_init().await;
    db_conn_init().await;
    init_tracer(
        "http://localhost:4317".to_string(),
        "feed-app".to_string()
    ).expect("TODO: panic message");

    //服务注册
    let client = Client::connect(["localhost:2379"], None).await?;
    let register = Register::new()
        .client(client)
        .lease(30)
        .keepalive(true)
        .register("/app/account_01", "http://127.0.0.1:50051");

        register.clone().start().await;


    let addr = "127.0.0.1:50051".parse().unwrap();
    println!("AccountServer listening on {}", addr);
    let account = AccountRpc::default();


    let r = Server::builder()
        .add_service(AccountServer::new(account))
        .serve(addr) //5.启动服务
        .await;

    if r.is_err() {
        register.clone().close().await;
        return Err(anyhow!("服务器启动失败 {:?}", r.err()));
    }

    global::shutdown_tracer_provider();

    Ok(())
}