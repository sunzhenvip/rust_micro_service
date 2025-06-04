use anyhow::{Result};
use crate::entities::wb_post::Model;
use crate::model::feed::find_feed_ids_by_uid_model;
use crate::model::post::find_posts_by_pids_model;


pub async fn feed_service(uid: u32, pid: u32, size: u8) -> Result<Vec<Model>>{
    let pids = find_feed_ids_by_uid_model(uid, pid, size).await?;
    if pids.is_empty() {
        return Ok(vec![]);
    }
    let posts = find_posts_by_pids_model(pids).await?;
    Ok(posts)
}