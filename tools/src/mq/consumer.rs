use std::future::Future;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

pub struct ConsumerBuilder {
    broker: Vec<String>,
    topic: String,
    group: String,
    pub consumer: Option<Consumer>,
}

impl ConsumerBuilder{

    pub fn new() -> Self {
        ConsumerBuilder {
            broker: vec![],
            topic: "".to_string(),
            group: "".to_string(),
            consumer: None,
        }
    }

    pub fn broker(mut self, broker: Vec<String>) -> Self {
        self.broker = broker;
        self
    }

    pub fn topic(mut self, topic: String) -> Self {
        self.topic = topic;
        self
    }

    pub fn group(mut self, group: String) -> Self {
        self.group = group;
        self
    }

    pub fn build(mut self) -> Self {
        tracing_subscriber::fmt::init();
        let con = Consumer::from_hosts(self.broker.clone())
            .with_topic(self.topic.clone())
            .with_group(self.group.clone())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .create().unwrap();
        self.consumer = Some(con);
        self
    }

    pub async fn poll<T, Fut>(self, cb: T) -> anyhow::Result<()>
        where
            T: Fn(Vec<u8>) -> Fut,
            Fut: Future<Output = anyhow::Result<()>> + Send,
    {
        let mut con = self.consumer.unwrap();
        loop {
            let mss = con.poll().unwrap();

            if mss.is_empty() {
                println!("No messages available right now.");
                continue;
            }
            for ms in mss.iter() {
                for m in ms.messages() {
                    let value = m.value.to_vec();
                    cb(value).await?;

                    // let array: Result<[u8; std::mem::size_of::<MyStruct>()], _> = TryFrom::try_from(m.value);
                    //
                    // let s = MyStruct::from_bytes(array.unwrap());
                    // println!(
                    //     "{}:{}@{}: {:?}",
                    //     ms.topic(),
                    //     ms.partition(),
                    //     m.offset,
                    //     s
                    // );
                }
                let _ = con.consume_messageset(ms);
            }
            con.commit_consumed()?;
        }
    }
}