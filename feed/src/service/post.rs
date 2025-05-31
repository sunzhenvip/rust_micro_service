use anyhow::{Result};
use tools::mq::producer::ProducerBuilder;
use crate::model::feed::create_feed;
use crate::model::post::{create_post};
use crate::types::post_message::PostMessage;
use crate::types::{Level, PID, UID};
use crate::utils::time::local_timestamp;


pub async fn create_post_service(uid: UID, level: Level, content: String) -> Result<PID>{

    //时间戳
    let now = local_timestamp();

    //插入数据库
    let pid = create_post(uid, content, now).await?;
    create_feed(uid, pid, now).await?;

    //发送到消息队列
   let mut producer =  ProducerBuilder::new()
        .hosts(vec!["127.0.0.1:9092".to_string()])
        .topic("quickstart")
        .ack_timeout(1)
        .build();

    //组装数据
    let post_message = PostMessage {
        uid,
        pid,
        level,
        created_time: now,
    };

    //发送到消息队列
    producer.send(&post_message.to_bytes()[..]).unwrap();

    Ok(pid)
}