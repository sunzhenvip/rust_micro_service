use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DbErr, EntityTrait, NotSet, QueryFilter, TransactionTrait};
use sea_orm::ActiveValue::Set;
use crate::model::get_db;
use crate::entities::wb_user;
use crate::entities::wb_user::Model;
use crate::entities::wb_user_info;
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

pub async fn create_user_model((u, uf): (User, UserInfo)) -> Result<u32, DbErr> {
    //开启事务
    let db = get_db();
    let txn = db.begin().await?;

    let now = local_timestamp();
    let user = wb_user::ActiveModel {
        uid: NotSet,
        phone: Set(u.phone),
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
    };

    let res = user_info.insert(&txn).await;

    if let Err(e) = res {
        let _ = txn.rollback().await;
        Err(e)
    } else {
        let _ = txn.commit().await;
        Ok(uid)
    }

}

pub async fn find(u: User) -> Result<Option<Model>, DbErr> {
    let db = get_db();
    let user = wb_user::Entity::find()
        .filter(
            Condition::all()
                .add(wb_user::Column::Phone.eq(u.phone))
                .add(wb_user::Column::Password.eq(u.password))
        )
        .one(db)
        .await?;
    Ok(user)
}

pub async fn find_by_uid_model(uid: u32) -> Result<Option<Model>, DbErr> {
    let db = get_db();
    let user = wb_user::Entity::find()
        .filter(
            Condition::all()
                .add(wb_user::Column::Uid.eq(uid))
        )
        .one(db)
        .await?;

    Ok(user)
}