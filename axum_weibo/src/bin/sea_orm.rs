#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数

// use crate::entities::users; // 这么写有问题
// 生成数据库表对应的实体结构体 并设置对应的生成路径
// sea-orm-cli generate entity -u mysql://root:123456@v@127.0.0.1:3306/test1 -o src/abc
use axum_weibo::entities::wb_user; // 可以这么写引入路径
use chrono::Local;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectOptions, Database, DbConn, DbErr, NotSet,
    QueryFilter, QueryOrder, TransactionTrait,
};
use sea_orm::{DbBackend, EntityTrait, Statement};
use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;

// 后面加的
use tracing_subscriber::fmt::time::FormatTime;
// use std::fmt::{self, Write};
use tracing_subscriber::fmt::format::Writer;

const DATABASE_URL: &str = "mysql://root:root@localhost:3306/seaorm";
// const DATABASE_URL: &str = "mysql://root:UUff98Y97hj@v@192.168.2.226:42730/test1";

async fn run() -> Result<DbConn, DbErr> {
    // 可选参数模式
    /*let mut opt = ConnectOptions::new(DATABASE_URL);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(std::time::Duration::from_secs(5)) // 链接尝试超时失败时间
        .sqlx_logging(false); // 开启 sqlx 日志;
    let db = Database::connect(opt).await?;*/
    // 默认链接模式
    let db = sea_orm::Database::connect(DATABASE_URL).await?;
    Ok(db)
}

// 自定义时间格式
struct LocalTime;

impl FormatTime for LocalTime {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

#[tokio::main]
async fn main() {
    // 开启打印日志
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();
    // 开启sql打印日志 可以设置更精细
    tracing_subscriber::fmt()
        .with_timer(LocalTime)
        .with_env_filter(
            EnvFilter::new("sea_orm=debug,sqlx=warn"), // 设置 sea_orm 为 debug，sqlx 为 warn
        )
        .init();
    if let Ok(db) = run().await {
        // let res = add_user(&db).await;
        // println!("add_user {:?}", res);
        // 根据id查询一条数据
        // let res = find(&db).await;
        // println!("find {:?}", res);
        // 查询全部数据
        // let res = find_all(&db).await;
        // println!("find {:?}", res);
        // 更新数据
        // let res = update(&db).await;
        // println!("find {:?}", res);
        // 删除数据
        // let res = delete(&db).await;
        // println!("find {:?}", res);
        // let _ = find_all_sql(&db).await;
        // let _ = find_where(&db).await;

        // let _ = transaction_user_update(&db).await;
        println!("链接成功")
    } else {
        println!("链接失败")
    }
}

// 新增数据到数据库
async fn add_user(db: &DbConn) -> Result<(), DbErr> {
    let user = wb_user::ActiveModel {
        uid: NotSet,
        phone: Set("15346231050".to_string()),
        password: Set("123456".to_string()),
        created_time: Set(Local::now().timestamp() as u32),
        updated_time: Set(Local::now().timestamp() as u32),
    };
    // let res = user.insert(db).await?; 这个语句会执行查询操作
    // println!("{:?}", res);
    // 这个语句只返回主键id
    let res = wb_user::Entity::insert(user).exec(db).await?;
    println!("插入成功, 插入的主键是: {:?}", res.last_insert_id);
    Ok(())
}

async fn find(db: &DbConn) -> Result<(), DbErr> {
    let user = wb_user::Entity::find_by_id(3u32).one(db).await?;
    dbg!(user.unwrap());
    Ok(())
}

async fn find_all(db: &DbConn) -> Result<(), DbErr> {
    let users = wb_user::Entity::find().all(db).await?;
    dbg!(users);
    Ok(())
}

async fn update(db: &DbConn) -> Result<(), DbErr> {
    let user = wb_user::ActiveModel {
        uid: Set(3),
        phone: Set("12333".to_string()),
        password: Set("88888".to_string()),
        created_time: Set(Local::now().timestamp() as u32),
        updated_time: Set(Local::now().timestamp() as u32),
    };
    let res = user.update(db).await?;
    println!("{:?}", res);
    Ok(())
}

async fn delete(db: &DbConn) -> Result<(), DbErr> {
    let user = wb_user::ActiveModel {
        uid: Set(4),
        ..Default::default()
    };
    let res = user.delete(db).await?;
    println!("{:?}", res);
    Ok(())
}

async fn find_all_sql(db: &DbConn) -> Result<(), DbErr> {
    let users = wb_user::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::MySql,
            r#"select * from wb_user where uid = ?"#,
            [5.into()],
        ))
        .all(db)
        .await?;
    dbg!(users);
    Ok(())
}

async fn find_where(db: &DbConn) -> Result<(), DbErr> {
    let users = wb_user::Entity::find()
        .filter(
            Condition::all()
                .add(wb_user::Column::Uid.eq(1))
                .add(wb_user::Column::Phone.like("%153%")),
        )
        .order_by_asc(wb_user::Column::Uid)
        .all(db)
        .await?;
    dbg!(users);
    Ok(())
}

// 事务sql
async fn transaction_user_update(db: &DbConn) -> Result<(), DbErr> {
    let txn = db.begin().await?;
    // 更新第一个
    wb_user::ActiveModel {
        uid: Set(1),
        phone: Set("2222".to_string()),
        password: Set("88888".to_string()),
        created_time: Set(Local::now().timestamp() as u32),
        updated_time: Set(Local::now().timestamp() as u32),
    }
    .update(&txn)
    .await?;
    // 更新第二个
    wb_user::ActiveModel {
        uid: Set(2),
        phone: Set("7777".to_string()),
        password: Set("88888".to_string()),
        created_time: Set(Local::now().timestamp() as u32),
        updated_time: Set(Local::now().timestamp() as u32),
    }
    .update(&txn)
    .await?;

    txn.commit().await?;
    println!("transaction_user_update 执行成功");
    Ok(())
}
