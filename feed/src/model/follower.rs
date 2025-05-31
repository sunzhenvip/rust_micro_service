use sea_orm::{EntityTrait, QuerySelect};
use crate::entities::wb_follower;
use crate::entities::wb_follower::Model;
use crate::model::get_db;
use crate::types::UID;
use anyhow::Result;
use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use crate::entities::wb_follower::Column;

pub async fn find_followers_by_uid(uid: UID) -> Result<Vec<Model>> {
    let db = get_db();
    let users = wb_follower::Entity::find()
        .column(Column::FollowerId)
        .filter(Column::Uid.eq(uid))
        .filter(Column::Status.eq(0))
        .all(db)
        .await?;

    Ok(users)
}