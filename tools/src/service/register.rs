use std::time::Duration;
use etcd_client::{Client, PutOptions};
use tokio::time;
use tokio::time::{Instant};

#[derive(Default, Clone)]
pub struct Register<'a>{
    client: Option<Client>,
    lease: i64,
    lease_id: Option<i64>,
    key: &'a str,
    value: &'a str,
    keep_alive: bool
}

impl<'a> Register<'a> {

    pub fn new() -> Self {
        Register::default()
    }

    pub fn client(mut self, client:Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn lease(mut self, lease: i64) -> Self {
        self.lease = lease;
        self
    }

    pub fn register<'b: 'a>(mut self, key: &'b str, value:  &'b str) -> Self {
        self.key = key;
        self.value = value;
        self
    }

    pub fn keepalive(mut self, ok: bool) -> Self {
        self.keep_alive = ok;
        self
    }

    pub async fn start(mut self) {
        println!("开始执行");
        if self.lease <= 0 {
            panic!("请设置租约");
        }
        let client = self.client.as_mut().unwrap();
        let resp = client.lease_grant(self.lease, None).await.unwrap();
        let id = resp.id();
        self.lease_id = Some(id);

        client.put(self.key, self.value, Some(PutOptions::new().with_lease(id))).await.unwrap();

        if self.keep_alive {
            let mut client_clone = client.clone();
            let lease = self.lease;
            tokio::spawn(async move {
                let d = (lease * 3 /4) as u64;
                let start = Instant::now() + Duration::from_secs(d);
                let interval = Duration::from_secs(d);
                let mut intv = time::interval_at(start, interval);

                loop {
                    intv.tick().await;
                    println!("开始续期 {:?}", id);
                    client_clone.lease_keep_alive(id).await.unwrap();
                }
            });
        }
    }

    pub async fn close(&mut self) {
        self.client.as_mut().unwrap().lease_revoke(self.lease_id.unwrap()).await.unwrap();
        println!("关闭了")
    }

}