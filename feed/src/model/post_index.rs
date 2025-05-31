use anyhow::Result;
use rand::Rng;
use sea_orm::{EntityName, EntityTrait, IntoActiveModel, QueryTrait};
use sea_orm::Order::Desc;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::Query;
use crate::entities::wb_post::{Column, Model as PostModel};
use crate::entities::wb_post_index::{Entity, Model, TableName};
use crate::entities::wb_post_index::Column::{CreatedTime, Pid, Uid};
use crate::model::get_db;
use crate::types::UID;

pub async fn create_post_index(sharding_id: u64, post: PostModel) -> Result<()> {
    let db = get_db();

    //指定分表
    let entity = Entity {
        table_name: TableName::from_str_truncate(format!("wb_post_index_{:02}", sharding_id)),
    };

    let post = Model{
        pid: post.pid,
        uid: post.uid,
        created_time: post.created_time,
    };
    let mut insert = Entity::insert(post.clone().into_active_model());
    insert.query().into_table(entity.table_ref());

    //插入数据
    insert.exec(db).await?.last_insert_id;
    Ok(())
}


pub async fn get_post_ids_by_uids(uids: Vec<UID>, time: u32, limit: u64) -> Result<Vec<Model>>{
    let db = get_db();

    let sharding_id = rand::thread_rng().gen_range(0..5);
    println!("{sharding_id}");

    //指定分表
    let entity = crate::entities::wb_post_index::Entity {
        table_name: TableName::from_str_truncate(format!("wb_post_index_{:02}", sharding_id)),
    };
    let mut select = Entity::find();

    let mut query = Query::select()
        .columns([
            Column::Pid,
            Column::Uid,
            Column::CreatedTime
        ])
        .from(entity.table_ref())
        .clone();
    if time != 0 {
        query.and_where(Expr::col(CreatedTime).lt(time));
    }
    let query = query
        .and_where(Expr::col(Uid).is_in(uids))
        .order_by(Pid, Desc)
        .limit(limit)
        .to_owned();


    *QueryTrait::query(&mut select) = query;

    let posts = select.clone().all(db).await?;
    Ok(posts)
}