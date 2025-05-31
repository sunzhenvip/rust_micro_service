use feed::proto::account::account_client::AccountClient;
use feed::proto::account::AccountRequest;
use tools::grpc::channel::ClientChannel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let channel = ClientChannel::new(200, "/app/account", "localhost:2379").await
        .connect().await?;

    let mut client = AccountClient::new(channel);

    let request = tonic::Request::new(AccountRequest { //3.请求
        uid: vec![5],
    });

    let response = client.get_users_by_uids(request).await; //4.调用
    let users = response.unwrap().into_inner().users;
    println!("RESPONSE={:?}", users);

    Ok(())
}