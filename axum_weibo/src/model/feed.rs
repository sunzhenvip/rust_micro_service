use sea_orm::{ColumnTrait, Condition, DbBackend, DbErr, EntityTrait, NotSet, Statement};
use sea_orm::ActiveValue::Set;
use sea_orm::QueryFilter;
use crate::entities::wb_feed;
use crate::model::get_db;

pub async fn create_many_feed_by_pid_model(pid: u32, uids: Vec<u32>) -> Result<(), DbErr> {
    if uids.is_empty() {
        return Ok(());
    }

    let db = get_db();

    let feeds: Vec<wb_feed::ActiveModel> = uids.into_iter().map(|uid| {
        wb_feed::ActiveModel {
            fid: NotSet,
            pid: Set(pid),
            uid: Set(uid)
        }
    }).collect();

    let _ = wb_feed::Entity::insert_many(feeds ).exec(db).await?;
    Ok(())

}

pub async fn delete_feed_by_pid_model(pid: u32) -> Result<(), DbErr> {
    let db = get_db();
    let _ = wb_feed::Entity::delete_many()
        .filter(wb_feed::Column::Pid.eq(pid))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn create_many_feed_by_uid_model(uid: u32, pids: Vec<u32>) -> Result<(), DbErr> {
    if pids.is_empty() {
        return Ok(());
    }

    let db = get_db();

    let feeds: Vec<wb_feed::ActiveModel> = pids.into_iter().map(|pid| {
        wb_feed::ActiveModel {
            fid: NotSet,
            pid: Set(pid),
            uid: Set(uid)
        }
    }).collect();
    let _ = wb_feed::Entity::insert_many(feeds ).exec(db).await?;
    Ok(())

}

pub async fn delete_feed_by_pids_model(follower: u32, pids: Vec<u32>) -> Result<(), DbErr> {
    let db = get_db();
    let _ = wb_feed::Entity::delete_many()
        .filter(
            Condition::all()
                .add(wb_feed::Column::Uid.eq(follower))
                .add(wb_feed::Column::Pid.is_in(pids))
        )
        .exec(db)
        .await?;

    Ok(())
}

pub async fn find_feed_ids_by_uid_model(uid: u32, pid: u32, size: u8) -> Result<Vec<u32>, DbErr>{

    let db = get_db();
    let posts =  wb_feed::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::MySql,
            r#"SELECT * FROM wb_feed WHERE uid=? and pid<?  order by pid desc limit ? "#,
            [uid.into(), pid.into(), size.into()],
        ))
        .all(db)
        .await?;

    if posts.is_empty() {
        return Ok(vec![]);
    }

    let pids: Vec<u32> = posts.iter().map(|m| m.pid).collect();
    Ok(pids)
}
