use anyhow::{Error, Result};
use crate::model::feed::{create_many_feed_by_pid_model, delete_feed_by_pid_model};
use crate::model::follower::{find_follower_ids_model};
use crate::model::post::{create_post_model, find_post_model, update_post_model};
use crate::utils::error::AppError;

pub async fn create_post_service(uid: u32, content: String) -> Result<u32>{

    let pid = create_post_model(uid, content).await?;


    //获取粉丝uids
    let fids = find_follower_ids_model(uid).await?;

    //插入feed表
    let _ = create_many_feed_by_pid_model(pid, fids).await?;

    Ok(pid)
}

pub async fn delete_post_service(uid: u32, pid: u32) -> Result<()>{
    let post = find_post_model(pid).await?;
    if post.is_none() {
        return Err(Error::from(AppError::new("微博不存在")));
    }

    let post = post.unwrap();

    if post.status == 0 {
        let _ = update_post_model(uid, pid).await?;
        //删除feed表
        delete_feed_by_pid_model(pid).await?;
    }

    Ok(())
}