use std::collections::HashSet;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DbBackend, DbErr, EntityTrait, NotSet, QueryFilter, Statement, TransactionTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{MysqlQueryBuilder, Query};
use crate::model::get_db;
use crate::entities::wb_user;
use crate::entities::wb_user_info;
use crate::entities::wb_user_info::Column::Uid;
use crate::entities::wb_user_info::Model as UserInfoModel;
use crate::entities::wb_user_phone;
use crate::entities::wb_user_phone::Model as UserPhoneModel;
use crate::utils::time::local_timestamp;

pub struct User {
    pub phone: String,
    pub password: String,
}

pub struct UserInfo {
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub gender: Option<u8>,
    pub birthday: Option<u32>
}

pub async fn create_user((u, uf): (User, UserInfo)) -> Result<u32, DbErr> {
    //开启事务
    let db = get_db();
    let txn = db.begin().await?;

    let now = local_timestamp();
    let user = wb_user::ActiveModel {
        uid: NotSet,
        phone: Set(u.phone.clone()),
        password: Set(u.password),
        created_time: Set(now),
        updated_time: Set(now)
    };

    let res = user.insert(&txn).await?;

    //构造user_info数据
    let uid = res.uid;
    let user_info = wb_user_info::ActiveModel {
        uid: Set(uid),
        nickname: Set(uf.nickname),
        avatar: Set(uf.avatar),
        gender: Set(uf.gender),
        birthday: Set(uf.birthday),
        updated_time: Set(now),

        level: Set(0),
        is_super: Set(0),
        follow_count: Set(0),
        fans_count: Set(0),
    };

    let res = user_info.insert(&txn).await;

    if let Err(e) = res {
        let _ = txn.rollback().await;
        return Err(e)
    }
    let _ = txn.commit().await;

    let user_phone = wb_user_phone::ActiveModel {
        id: NotSet,
        phone: Set(u.phone.clone()),
        uid: Set(uid),
    };
    user_phone.insert(db).await?;

    Ok(uid)

}

pub async fn find(u: User) -> Result<Option<UserInfoModel>, DbErr> {
    let db = get_db();
    let user = wb_user::Entity::find()
        .filter(
            Condition::all()
                .add(wb_user::Column::Phone.eq(u.phone))
                .add(wb_user::Column::Password.eq(u.password))
        )
        .one(db)
        .await?;

    let user = wb_user_info::Entity::find()
        .filter(
            Condition::all()
                .add(wb_user_info::Column::Uid.eq(user.unwrap().uid))
        )
        .one(db)
        .await?;
    Ok(user)
}


pub async fn find_user_by_phone(phone: String) -> Result<Option<UserPhoneModel>, DbErr> {
    let db = get_db();
    let user = wb_user_phone::Entity::find()
        .filter(
            Condition::all()
                .add(wb_user_phone::Column::Phone.eq(phone))
        )
        .one(db)
        .await?;

    println!("{:?}", user);
    Ok(user)
}

pub async fn find_by_uids(uids: HashSet<u32>) -> Result<Vec<UserInfoModel>, DbErr> {
    let db = get_db();

    let query = Query::select()
        .from(wb_user_info::Entity)
        .columns([
            (wb_user_info::Entity, wb_user_info::Column::Uid),
            (wb_user_info::Entity, wb_user_info::Column::Level),
            (wb_user_info::Entity, wb_user_info::Column::IsSuper),
            (wb_user_info::Entity, wb_user_info::Column::FollowCount),
            (wb_user_info::Entity, wb_user_info::Column::FansCount),
            (wb_user_info::Entity, wb_user_info::Column::Nickname),
            (wb_user_info::Entity, wb_user_info::Column::Avatar),
            (wb_user_info::Entity, wb_user_info::Column::UpdatedTime),
        ])
        .and_where(Expr::col(Uid).is_in(uids))
        .to_owned();
    let sql = query.to_string(MysqlQueryBuilder);

    let users: Vec<UserInfoModel> = wb_user_info::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::MySql,
            sql,
            [],
        ))
        .all(db)
        .await?;

    Ok(users)
}