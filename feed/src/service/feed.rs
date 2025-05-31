use std::collections::{HashMap};
use crate::entities::wb_post::Model;
use crate::model::followee::get_followee_ids_by_uid;
use crate::model::post::{get_posts_by_pids};
use crate::types::{PID, UID};
use anyhow::{Result};
use opentelemetry::{Context, global, KeyValue};
use opentelemetry::trace::{SpanKind, TraceContextExt};
use serde::Serialize;
use tools::grpc::channel::ClientChannel;
use tools::trace::InjectorMetadataMap;
use crate::model::feed::{get_feed_ids_by_uid};
use crate::model::post_index::get_post_ids_by_uids;
use crate::proto::account::account_client::AccountClient;
use crate::proto::account::account_reply::User;
use crate::proto::account::AccountRequest;
use opentelemetry::trace::Tracer;

#[derive(Serialize)]
pub struct Feed {
    pid: u64,
    uid: u32,
    content: String,
    created_time: u32,
    nickname: String,
    avatar: String,
    level: u8,
    follow_count: u16,
    fans_count: u32,
}

pub async fn get_feeds_service(uid: UID, is_super: u8, time: u32) -> Result<Vec<Feed>>{
    let mut posts = vec![];

    //只使用推模式
    if is_super == 1 {
        posts = get_feeds_by_push(uid, time, 20).await?;
    } else {
        let push_feeds = get_feeds_by_push(uid, time, 10).await?;
        let pull_feeds =  get_feeds_by_pull(uid, time, 10).await?;

        posts = push_feeds.into_iter().chain(pull_feeds.into_iter()).collect::<Vec<Model>>();
    }

    //获取用户id
    let uids: Vec<UID> = posts.iter().map(|x| {
        x.uid
    }).collect();
    let user_map = get_users_by_grpc(uids).await?;
    println!("user_map {:?}", user_map);

    posts.sort_by(|x, y| y.created_time.cmp(&x.created_time) );


    //调用用户获取grpc
    let feeds: Vec<Feed> = posts.iter().map(|x| {
        let user = user_map.get(&x.uid).unwrap();
        Feed{
            pid: x.pid,
            uid: x.uid,
            content: x.content.clone(),
            created_time: x.created_time,
            nickname: user.nickname.clone(),
            avatar: user.avatar.clone(),
            level: user.level as u8,
            follow_count: user.follow_count as u16,
            fans_count: user.fans_count,
        }
    }).collect();

    Ok(feeds)
}

async fn get_users_by_grpc(uids: Vec<UID>) -> Result<HashMap<UID, User>>{
    let tracer = global::tracer("feed-app");
    let span = tracer
        .span_builder(String::from("Greeter/client"))
        .with_kind(SpanKind::Client)
        .with_attributes(vec![KeyValue::new("component", "grpc")])
        .start(&tracer);
    let cx = Context::current_with_span(span);



    let channel = ClientChannel::new(200, "/app/account", "localhost:2379").await
        .connect().await?;

    let mut client = AccountClient::new(channel);

    let mut request = tonic::Request::new(AccountRequest { //3.请求
        uid: uids,
    });

    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&cx, &mut InjectorMetadataMap(request.metadata_mut()))
    });


    let response = client.get_users_by_uids(request).await?; //4.调用
    let users = response.into_inner().users;
    let users:HashMap<UID, User> = users.into_iter().map(|x| {
        (x.uid, x)
    }).collect();

    cx.span().add_event(
        "Got response!".to_string(),
        vec![KeyValue::new("status", "猩猩自习室")],
    );
    Ok(users)
}

async fn get_feeds_by_pull(uid: UID, time: u32, limit: u64) -> Result<Vec<Model>>{
    //查询关注大V用户
    let followee_ids = get_followee_ids_by_uid(uid).await?;
    let followee_ids: Vec<UID> =  followee_ids.iter().map(|x| {
        x.followee_id
    }).collect();

    let post_ids = get_post_ids_by_uids(followee_ids, time, limit).await;
    if post_ids.is_err() {
        println!("followee_ids {:?}", post_ids.err());
        return Ok(vec![])
    }

    let post_ids: Vec<PID> = post_ids.unwrap().iter().map(|x| {
        x.pid
    }).collect();

    if post_ids.is_empty() {
        return Ok(vec![])
    }

    //先查本地缓存
    //差查redis



    //查询动态
    //TODO 可以继续优化逻辑，不直接从post表读取内容，可以建一个post索引表
    //TODO 读取缓存和本地缓存
    //TODO 读取批量改成小批量
    get_posts_by_pids(post_ids, time, limit, true).await

}

async fn get_feeds_by_push(uid: UID, time: u32, limit: u64) -> Result<Vec<Model>>{
    //获取用户的收件箱
    let feed_ids =  get_feed_ids_by_uid(uid, time).await?;
    let post_ids:Vec< PID > = feed_ids.iter().map(|x|{
        x.pid
    }).collect();
    //查询动态

    if post_ids.is_empty() {
        return Ok(vec![])
    }

    let mut posts = vec![];
    println!("post_ids {:?}", post_ids);
    //TODO
    for pid in post_ids {
        let r = get_posts_by_pids(vec![pid], time, limit, false).await.unwrap();
        if r.is_empty() {
            continue
        }
        let post = r.get(0).unwrap().clone();
        posts.push(post)
    }
    Ok(posts)
}

