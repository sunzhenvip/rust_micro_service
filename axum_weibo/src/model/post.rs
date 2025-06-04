use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DbBackend, DbErr, EntityTrait, NotSet, QueryFilter, Statement, Value};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{MysqlQueryBuilder, Query};
use crate::entities::{wb_post};
use crate::entities::wb_post::Column::Pid;
use crate::entities::wb_post::Model;
use crate::model::get_db;
use crate::utils::time::local_timestamp;

//获取粉丝uid
pub async fn find_post_model(pid: u32) -> Result<Option<Model>, DbErr>{

    let db = get_db();
    let post =  wb_post::Entity::find()
        .filter(
            Condition::all()
                .add(wb_post::Column::Pid.eq(pid))
        )
        .one(db)
        .await?;
    Ok(post)
}

pub async fn find_pids_by_uid_model(uid: u32) -> Result<Vec<u32>, DbErr>{

    let db = get_db();
    let posts =  wb_post::Entity::find()
        .filter(
            Condition::all()
                .add(wb_post::Column::Uid.eq(uid))
                .add(wb_post::Column::Status.eq(0))
        )
        .all(db)
        .await?;
    if posts.is_empty() {
        return Ok(vec![]);
    }

    let pids: Vec<u32> = posts.iter().map(|p| p.pid ).collect();
    Ok(pids)
}

pub async fn create_post_model(uid: u32, content: String) -> Result<u32, DbErr> {
    let db = get_db();
    //时间戳
    let now = local_timestamp();
    //构造数据
    let post = wb_post::ActiveModel{
        pid: NotSet,
        uid: Set(uid),
        content: Set(content),
        status: Set(0),
        created_time: Set(now) ,
        updated_time: Set(now),
    };
    let res = post.insert(db).await?;
    Ok(res.pid)
}

pub async fn update_post_model(uid: u32, pid: u32) -> Result<(), DbErr> {
    let db = get_db();
    let _ = wb_post::Entity::update_many()
        .col_expr(wb_post::Column::Status, Expr::value(Value::TinyUnsigned(Some(1))))
        .col_expr(wb_post::Column::UpdatedTime, Expr::value(Value::Unsigned(Some(local_timestamp()))))
        .filter(
            Condition::all()
                .add(wb_post::Column::Uid.eq(uid))
                .add(wb_post::Column::Pid.eq(pid))
        )
        .exec(db)
        .await?;

    Ok(())
}

pub struct Post {
    pub pid: u32,
    pub uid: u32,
    pub content: String,
    pub created_time: u32,
}

pub async fn find_posts_by_pids_model(feed_ids: Vec<u32>) -> Result<Vec<Model>, DbErr>{
    let db = get_db();

    let query = Query::select()
        .from(wb_post::Entity)
        .columns([
            (wb_post::Entity, wb_post::Column::Pid),
            (wb_post::Entity, wb_post::Column::Uid),
            (wb_post::Entity, wb_post::Column::Content),
            (wb_post::Entity, wb_post::Column::Status),
            (wb_post::Entity, wb_post::Column::CreatedTime),
            (wb_post::Entity, wb_post::Column::UpdatedTime),
        ])
        .and_where(Expr::col(Pid).is_in(feed_ids))
        .to_owned();
    let sql = query.to_string(MysqlQueryBuilder);

    let posts: Vec<Model> = wb_post::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::MySql,
            sql,
            [],
        ))
        .all(db)
        .await?;

    Ok(posts)

}