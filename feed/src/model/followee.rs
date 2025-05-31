use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use crate::entities::{wb_followee};
use crate::entities::wb_followee::{Column, Model};
use crate::model::get_db;
use crate::types::UID;
use anyhow::Result;

pub async fn get_followee_ids_by_uid(uid: UID) -> Result<Vec<Model>>{
    let db = get_db();
    let users = wb_followee::Entity::find()
        .column(Column::FolloweeId)
        .filter(Column::FolloweeLevel.gt(0))
        .filter(Column::Uid.eq(uid))
        .filter(Column::Status.eq(0))
        .all(db)
        .await?;

    Ok(users)
}