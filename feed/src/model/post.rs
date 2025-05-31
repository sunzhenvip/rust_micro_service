use sea_orm::{EntityName, EntityTrait, IntoActiveModel, QueryTrait};
use sea_orm::sea_query::{Expr, Query, types::Order::Desc};
use crate::entities::wb_feed::Column::CreatedTime;
use crate::entities::wb_post::{Column, Entity, Model, TableName};
use crate::entities::wb_post::Column::{Pid, Status};
use crate::model::get_db;
use crate::types::{PID, UID};
use anyhow::Result;
use rand::Rng;
use snowflaker::generator::{Generator, SnowflakeGenerator};

pub async fn create_post(uid: UID, content: String, now: u32) -> Result<PID> {
    let db = get_db();


    //生成pid
    let gen = SnowflakeGenerator::new(31, 31);
    let pid = gen.as_ref().unwrap().next_id().unwrap();
    println!("{:?}", pid);

    let sharding_id = pid  % 5;

    //指定分表
    let entity = Entity {
        table_name: TableName::from_str_truncate(format!("wb_post_{:02}", sharding_id)),
    };

    let post = Model{
        pid,
        uid,
        content,
        status: 0,
        created_time: now,
        updated_time: now,
    };
    let mut insert = Entity::insert(post.clone().into_active_model());
    insert.query().into_table(entity.table_ref());

    //插入数据
    let pid = insert.exec(db).await?.last_insert_id;
    Ok(pid)
}

pub async fn get_post_by_pid(pid: PID) -> anyhow::Result<Model>{
    let db = get_db();

    let sharding_id = pid % 5;
    println!("{sharding_id}");

    //指定分表
    let entity = Entity {
        table_name: TableName::from_str_truncate(format!("wb_post_{:02}", sharding_id)),
    };
    let mut select = Entity::find_by_id(pid);

    *QueryTrait::query(&mut select) = Query::select()
        .columns([
            Column::Pid,
            Column::Uid,
            Column::Content,
            Column::CreatedTime,
            Column::UpdatedTime,
            Column::Status
        ])
        .from(entity.table_ref())
        .and_where(Expr::col(Pid).eq(pid))
        .and_where(Expr::col(Status).eq(0))
        .to_owned();

    let post = select.clone().one(db).await?.unwrap();
    Ok(post)
}


pub async fn clone_post(sharding_id: u64, post: Model) -> Result<PID> {
    let db = get_db();

    //指定分表
    let entity = Entity {
        table_name: TableName::from_str_truncate(format!("wb_post_{:02}", sharding_id)),
    };

    let mut insert = Entity::insert(post.clone().into_active_model());
    insert.query().into_table(entity.table_ref());

    //插入数据
    let pid = insert.exec(db).await?.last_insert_id;
    Ok(pid)
}

pub async fn get_posts_by_pids(pids: Vec<PID>, time: u32, limit: u64, is_shard: bool) -> Result<Vec<Model>>{
    let db = get_db();

    let mut sharding_id = 0;
    if is_shard {
        sharding_id = rand::thread_rng().gen_range(0..5);
    } else {
        sharding_id = pids[0] % 5;
    }

    println!("sharding_id {sharding_id}");

    //指定分表
    let entity = Entity {
        table_name: TableName::from_str_truncate(format!("wb_post_{:02}", sharding_id)),
    };
    let mut select = Entity::find();

    let mut query = Query::select()
        .columns([
            Column::Pid,
            Column::Uid,
            Column::Content,
            Column::CreatedTime,
            Column::UpdatedTime,
            Column::Status
        ])
        .from(entity.table_ref())
        .clone();
    if time != 0 {
        query.and_where(Expr::col(CreatedTime).lt(time));
    }
    let query = query
        .and_where(Expr::col(Status).eq(0))
        .and_where(Expr::col(Pid).is_in(pids))
        .order_by(Column::CreatedTime, Desc)
        .limit(limit)
        .to_owned();


    *QueryTrait::query(&mut select) = query;

    let posts = select.clone().all(db).await?;
    Ok(posts)
}