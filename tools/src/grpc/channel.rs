use tokio::sync::mpsc::Sender;
use tonic::transport::{Channel, Endpoint};
use tower::discover::Change;
use crate::service::discovery::Discovery;
use anyhow::Result;
use etcd_client::{Client, EventType};

pub struct ClientChannel {
    channel: Channel,
    sender: Sender<Change<String, Endpoint>>,
    service: String,
    discovery: Discovery,
}

impl ClientChannel {

    pub async fn new(capacity: usize, service: &str, etcd_host: &str) -> Self {
        let (channel, sender) = Channel::balance_channel(capacity);

        let etcd_client = Client::connect([etcd_host], None).await.unwrap();
        let discovery = Discovery::new(etcd_client);
        ClientChannel {
            channel,
            sender,
            service: service.to_string(),
            discovery,
        }
    }

    pub async fn connect(mut self) -> Result<Channel>{

        //获取
        let sls = self.discovery.service_list(self.service.as_str()).await;
        println!("{:?}", sls);

        for (key, value) in sls {
            println!("{} {}", key, value);
            let change = Change::Insert(key, Endpoint::from_shared(value).unwrap());
            self.sender.send(change).await.unwrap();
        }

        //监听
        tokio::spawn(async move {
            let mut r = self.discovery.watch_service(self.service.as_str()).await;
            while let Some(e) = r.recv().await {

                println!("received: {:?}", e.event_type());

                let kv = e.kv().unwrap().clone();
                let (key, value) = kv.into_key_value();
                let key = String::from_utf8(key).unwrap(); // Converting to string
                let value = String::from_utf8(value).unwrap(); // Converting to string
                println!("{:?} {:?}", key, value);

                if EventType::Put == e.event_type() {
                    let change = Change::Insert(key.clone(), Endpoint::from_shared(value).unwrap());
                    self.sender.send(change).await.unwrap();
                }

                if EventType::Delete == e.event_type() {
                   let change =  Change::<String, Endpoint>::Remove(key.clone());
                    self.sender.send(change).await.unwrap();
                }

            }
        });

        let chan = self.channel.clone();
        Ok(chan)
    }
}