use anyhow::anyhow;
use feed::config::config_init;
use feed::model::db_conn_init;
use feed::model::feed::create_feed;
use feed::model::follower::find_followers_by_uid;
use feed::model::post::{clone_post, get_post_by_pid};
use feed::model::super_follower::find_super_followers_by_uid;
use feed::types::post_message::PostMessage;
use tools::mq::consumer::ConsumerBuilder;
use anyhow::Result;
use feed::entities::wb_post::Model;
use feed::model::post_index::create_post_index;
use feed::types::{Level, PID, UID};

#[tokio::main]
async fn main() {
    config_init().await;
    db_conn_init().await;

    let c = ConsumerBuilder::new()
        .broker(vec!["127.0.0.1:9092".to_string()])
        .group("post_group".to_string())
        .topic("quickstart".to_string())
        .build();
    c.poll(handle_message).await.unwrap();
}

async fn handle_message(value: Vec<u8>) -> Result<()> {
    let array: Result<[u8; std::mem::size_of::<PostMessage>()], _> = TryFrom::try_from(value);
    // println!("{:?}", array);
    let m = PostMessage::from_bytes(array.unwrap());
    let PostMessage{ uid, pid ,level, created_time} = m;

    //如果是大V则，存到多个表的里面
    if level > 0 {
        let _ = handle_post(pid).await;
    }
    let _ = handle_feed(uid, pid, level, created_time).await;
    Ok(())
}

async fn handle_feed(uid: UID, pid: PID, level: Level, created_time: u32) -> Result<()> {
//推模式入库到收件箱
    let mut followers = vec![];
    //大V
    if level > 0 {

        let res = find_super_followers_by_uid(uid).await?;
        followers = res.iter().map(|m| {
            m.follower_id
        }).collect();

    } else {

        let res = find_followers_by_uid(uid).await?;
        followers = res.iter().map(|m| {
            m.follower_id
        }).collect();

    }

    println!("followers {:?}", followers);

    if followers.is_empty() {
        println!("没有follower");
        return Err(anyhow!("没有follower"));
    }

    //插入到数据库，可以根据follower uid筛选出一批路由到同一个表的数据，再批量插入
    for uid in followers {
        create_feed(uid, pid, created_time).await?;
    }

    Ok(())
}

async fn handle_post_index(sharding_id: u64, post: Model) -> Result<()> {
    create_post_index(sharding_id, post).await
}

async fn handle_post(pid: PID) -> Result<()> {
    //查询post内容
    let post = get_post_by_pid(pid).await?;
    let sharding_id = pid % 5;

    for num in 0..5 {
        if num != sharding_id {
            clone_post(num, post.clone()).await?;
        }
        //大V的动态会全部发
        handle_post_index(num, post.clone()).await?;
    }
    Ok(())
}