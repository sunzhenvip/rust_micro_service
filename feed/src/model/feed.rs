use sea_orm::{EntityName, EntityTrait, IntoActiveModel, QueryTrait};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::Query;
use crate::entities::wb_feed;
use crate::entities::wb_feed::{Entity, Model, TableName};
use crate::model::get_db;
use crate::types::{FID, PID, UID};
use anyhow::{anyhow, Result};
use crate::entities::wb_feed::Column::{CreatedTime, Fid, Pid, Uid};

pub async fn create_feed(uid: UID, pid: PID, created_time: u32) -> Result<FID> {
    let db = get_db();

    let sharding_id = uid % 5;

    //指定分表
    let entity = Entity {
        table_name: TableName::from_str_truncate(format!("wb_feed_{:02}", sharding_id)),
    };

    let feed = wb_feed::Model{
        fid: 0,
        pid,
        uid,
        created_time,
    };
    let mut insert = Entity::insert(feed.clone().into_active_model());
    insert.query().into_table(entity.table_ref());

    //插入数据
    let fid = insert.exec(db).await?.last_insert_id;
    Ok(fid)
}


pub async fn get_feed_ids_by_uid(uid: UID, time: u32) -> Result<Vec<Model>>{
    let db = get_db();

    let sharding_id = uid % 5;
    println!("sharding_id {sharding_id}");

    //指定分表
    let entity = Entity {
        table_name: TableName::from_str_truncate(format!("wb_feed_{:02}", sharding_id)),
    };
    let mut select = Entity::find();

    let mut query = Query::select()
        .column(Fid)
        .column(Uid)
        .column(Pid)
        .column(CreatedTime)
        .from(entity.table_ref())
        .clone();
        if time != 0 {
            query.and_where(Expr::col(CreatedTime).lt(time));
        }
        let query = query
            .and_where(Expr::col(Uid).eq(uid))
            .to_owned();

    *QueryTrait::query(&mut select) = query;

    let feeds = select.clone().all(db).await;
    if let Err(e) = feeds {
        println!("feed {:?}", e);
        return Err(anyhow!("hh"))
    } else {
        let feeds = feeds.unwrap();
        println!("feed {:?}", feeds);
        Ok(feeds)
    }

}