use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DbErr, EntityTrait, NotSet, QueryFilter, Value};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;
use crate::entities::{wb_follower};
use crate::entities::wb_follower::Model;
use crate::model::get_db;
use crate::utils::time::local_timestamp;


//获取粉丝uid
pub async fn find_follower_ids_model(followee: u32) -> Result<Vec<u32>, DbErr>{

    let db = get_db();
    let uids =  wb_follower::Entity::find()
        .filter(
            Condition::all()
                .add(wb_follower::Column::FolloweeId.eq(followee))
        )
        .all(db)
        .await?;

    if uids.is_empty() {
        return Ok(vec![]);
    }

    let arr: Vec<u32> = uids.iter().map(| v | v.follower_id ).collect();
    Ok(arr)

}

pub async fn find_follower_model(follower: u32, followee: u32) -> Result<Option<Model>, DbErr> {

    let db = get_db();
    let res =  wb_follower::Entity::find()
        .filter(
            Condition::all()
                .add(wb_follower::Column::FollowerId.eq(follower))
                .add(wb_follower::Column::FolloweeId.eq(followee))
        )
        .one(db)
        .await?;
    Ok(res)
}


pub async fn create_follow_model(follower: u32, followee: u32) -> Result<(), DbErr> {
    let db = get_db();
    //时间戳
    let now = local_timestamp();
    //构造数据
    let user = wb_follower::ActiveModel{
        id: NotSet,
        follower_id: Set(follower),
        followee_id: Set(followee),
        status: Set(0),
        created_time: Set(now) ,
        updated_time: Set(now),
    };
    let _ = user.insert(db).await?;
    Ok(())
}

pub async fn update_follow_model(follower: u32, followee: u32, status: u8) -> Result<(), DbErr> {
    let db = get_db();
    let _ = wb_follower::Entity::update_many()
        .col_expr(wb_follower::Column::Status, Expr::value(Value::TinyUnsigned(Some(status))))
        .col_expr(wb_follower::Column::UpdatedTime, Expr::value(Value::Unsigned(Some(local_timestamp()))))
        .filter(
            Condition::all()
                .add(wb_follower::Column::FollowerId.eq(follower))
                .add(wb_follower::Column::FolloweeId.eq(followee))
        )
        .exec(db)
        .await?;

    Ok(())
}