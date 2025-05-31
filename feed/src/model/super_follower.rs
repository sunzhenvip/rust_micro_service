use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::entities::wb_super_follower;
use crate::entities::wb_super_follower::{Column, Model};
use crate::model::get_db;
use crate::types::UID;
use anyhow::Result;

pub async fn find_super_followers_by_uid(uid: UID) -> Result<Vec<Model>> {
    let db = get_db();
    let users = wb_super_follower::Entity::find()
        .filter(Column::Uid.eq(uid))
        .all(db)
        .await?;

    Ok(users)
}