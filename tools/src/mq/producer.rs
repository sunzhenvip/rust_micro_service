use std::time::Duration;
use kafka::client::RequiredAcks;
use kafka::producer::{Producer, Record};

pub struct  ProducerBuilder {
    hosts: Vec<String>,
    ack_timeout: u64,
    topic: String,
    producer: Option<Producer>
}

impl ProducerBuilder {
    pub fn new() -> Self {
        ProducerBuilder {
            hosts: vec![],
            ack_timeout: 0,
            topic: "".to_string(),
            producer: None,
        }
    }

    pub fn hosts(mut self, hosts: Vec<String>) -> Self {
        self.hosts = hosts;
        self
    }

    pub fn topic(mut self, topic: &str) -> Self {
        self.topic = topic.to_string();
        self
    }

    pub fn ack_timeout(mut self, timeout: u64) -> Self {
        self.ack_timeout = timeout;
        self
    }

    pub fn build(mut self) -> Self {
        let producer = Producer::from_hosts(self.hosts.clone())
            .with_ack_timeout(Duration::from_secs(self.ack_timeout))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();
        self.producer = Some(producer);
        self
    }

    pub fn send(&mut self, data: &[u8]) -> kafka::error::Result<()> {
        let producer = self.producer.as_mut().unwrap();
         producer.send(&Record::from_value(self.topic.as_str(), data))
    }
}