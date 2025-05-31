use std::collections::HashMap;
use etcd_client::{Client, Event, GetOptions, WatchOptions};
use tokio::sync;
use tokio::sync::mpsc::Receiver;

type ServiceList = HashMap<String, String>;

#[derive(Clone)]
pub struct Discovery{
    client: Client
}

impl Discovery {

    pub fn new(client: Client) -> Self {
        Discovery {
            client
        }
    }

    pub async fn service_list(&mut self, key: &str) -> ServiceList {
        let resp = self.client.get(
            key,
            Some(GetOptions::new().with_prefix())
        ).await.unwrap();

        let mut sl = HashMap::new();
        for kv in resp.kvs() {
            sl.insert(
                kv.key_str().unwrap().to_string(),
                kv.value_str().unwrap().to_string()
            );
        }
        sl
    }

    pub async fn watch_service(&mut self, key:&str) -> Receiver<Event> {
        let (_watcher, mut stream) = self.client.watch(
            key,
            Some(WatchOptions::new().with_prefix())
        ).await.unwrap();

        let (sender, reader) = sync::mpsc::channel(10);

        tokio::spawn(async move{
            while let Some(resp) = stream.message().await.unwrap() {
                if resp.created() {
                    println!("watcher created: {}", resp.watch_id());
                }

                if resp.canceled() {
                    println!("watch canceled: {}", resp.watch_id());
                }

                for event in resp.events() {

                    sender.send(event.clone()).await.expect("TODO: panic message");

                }
            }
        });
        reader
    }
}