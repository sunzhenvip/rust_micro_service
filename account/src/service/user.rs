use std::collections::{HashSet};
use anyhow::{Error, Result};
use chrono::{Duration, Local};
use rustis::client::Client;
use rustis::commands::{SetCondition, SetExpiration, StringCommands};
use rustis::resp::BulkString;
use serde::{Deserialize, Serialize};
use crate::config::local_cache::LOCAL_CACHE;
use crate::handler::user::{ReqCreateUser, ReqLogin};
use crate::model;
use crate::model::user::{create_user, find, find_user_by_phone, User, UserInfo};
use crate::utils::crypto::md5;
use crate::utils::error::AppError;
use crate::utils::jwt::{Claims, create_jwt};

pub async fn create_user_service(ruser: ReqCreateUser) -> Result<u32> {
    let user = User {
        phone: ruser.phone.clone(),
        password: md5(&ruser.password, "123")
    };

    //验证手机号有没有注册
    let user_phone = find_user_by_phone(ruser.phone.clone()).await.unwrap();
    if user_phone.is_some() {
        return Err(Error::from(AppError::new("已经注册了")));
    }

    let user_info = UserInfo {
        nickname: ruser.nickname,
        avatar: ruser.avatar,
        gender: ruser.gender,
        birthday: ruser.birthday
    };

    let uid= create_user((user, user_info)).await?;
    Ok(uid)
}

pub async fn login_service(login: ReqLogin) -> Result<String> {

    let user = User {
        phone: login.phone.clone(),
        password: md5(&login.password, "123")
    };

    //判断是否有此用户
    let user_phone = find_user_by_phone(login.phone.clone()).await?;
    if user_phone.is_none() {
        return Err(Error::from(AppError::new("没有找到用户信息")));
    }

    let res = find(user).await?;

    if res.is_none() {
        return Err(Error::from(AppError::new("没找到用户信息")));
    }

    let user = res.unwrap();
    //创建jwt
    let now = Local::now();

    let c = Claims {
        uid: user.uid,
        level: user.level,
        is_super: user.is_super,
        exp: (now + Duration::hours(24)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    let token = create_jwt(c).unwrap();
    Ok(token)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RspUser {
    pub uid: u32,
    pub level: u8,
    pub follow_count: u16,
    pub fans_count: u32,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
}

pub async fn get_users_by_uids_service(uids: Vec<u32>) -> Result<Vec<RspUser>>{
    if uids.is_empty()  {
        return Err(Error::from(AppError::new("参数为空")));
    }

    println!("{:?}", uids);

    let mut v = Vec::new();

    //获取本地缓存
    // let cache:Cache<u32, RspUser> = Cache::builder()
    //     .name("account")
    //     .max_capacity(10000)
    //     .time_to_idle(std::time::Duration::from_secs(300))
    //     .build();

    let mut uncached_uids = HashSet::new();
    for uid in uids {
        let r = LOCAL_CACHE.get().unwrap().get(&uid);
        if r.is_some() {
            println!("来自 localcache {}", uid);
            v.push(r.unwrap());
        } else {
            uncached_uids.insert(uid);
        }
    }

    if uncached_uids.is_empty() {
        return Ok(v);
    }

    //获取redis
    let client = Client::connect("127.0.0.1:16379").await?;
    let mut unredis_uids = HashSet::new();
    for uid in uncached_uids {
        let r: Result<BulkString, rustis::Error> = client.get(format!("act_{:?}", uid).as_str()).await;
        if let Ok(bulk) = r {
            if bulk.is_empty() {
                unredis_uids.insert(uid);
            } else {
                let deserialized: RspUser = serde_json::from_slice(&bulk)?;
                println!("来自 redis {:?}", deserialized);
                v.push(deserialized.clone());
                LOCAL_CACHE.get().unwrap().insert(uid, deserialized.clone());
            }
        }
    }

    if unredis_uids.is_empty() {
        return Ok(v);
    }

    //获取mysql
    let users = model::user::find_by_uids(unredis_uids).await?;

    for user in users {
        let rsp_user = RspUser {
            uid: user.uid,
            level: user.level,
            follow_count: user.follow_count,
            fans_count: user.fans_count,
            nickname: user.nickname.clone(),
            avatar: user.avatar.clone(),
        };
        v.push(rsp_user.clone());

        let serialized = serde_json::to_vec(&rsp_user)?;
        let _ = client.set_with_options(
            format!("act_{}", user.uid),
            serialized,
            SetCondition::None,
            SetExpiration::Ex(60 * 60),
            false).await;

        LOCAL_CACHE.get().unwrap().insert(user.uid, rsp_user.clone());
    }
    Ok(v)
}