use anyhow::{Error, Result};
use crate::model::feed::{create_many_feed_by_uid_model, delete_feed_by_pids_model};
use crate::model::follower::{create_follow_model, find_follower_model, update_follow_model};
use crate::model::post::find_pids_by_uid_model;
use crate::model::user::{find_by_uid_model};
use crate::utils::error::AppError;

pub async fn follow_service(follower: u32, followee: u32) -> Result<()>{
    let user = find_by_uid_model(followee).await?;
    if user.is_none() {
        return Err(Error::from(AppError::new("用户不存在")));
    }

    //查询是否有数据
    let res = find_follower_model(follower, followee).await?;
    if res.is_none() {
        let _ = create_follow_model(follower, followee).await?;
    } else {
        //如果status为1就去更新
        let m = res.unwrap();
        if m.status == 0 {
            return Ok(());
        }
        let _ = update_follow_model(follower, followee, 0).await?;
    }

    let pids = find_pids_by_uid_model(followee).await?;
    let _ = create_many_feed_by_uid_model(follower, pids).await?;

    Ok(())
}

pub async fn unfollow_service(follower: u32, followee: u32) -> Result<()>{
    let user = find_by_uid_model(followee).await?;
    if user.is_none() {
        return Err(Error::from(AppError::new("用户不存在")));
    }

    //判断是否有数据
    let res = find_follower_model(follower, followee).await?;
    if let Some(m) = res {
        if m.status == 0 {
            let _ = update_follow_model(follower, followee, 1).await?;

            let pids = find_pids_by_uid_model(followee).await?;
            //删除feed表里的数据
            let _ = delete_feed_by_pids_model(follower, pids).await?;
        }
    }

    Ok(())
}