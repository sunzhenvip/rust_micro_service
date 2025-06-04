use anyhow::{Error, Result};
use chrono::{Duration, Local};
use crate::handler::user::{ReqCreateUser, ReqLogin};
use crate::model::user::{create_user_model, find, User, UserInfo};
use crate::utils::crypto::md5;
use crate::utils::error::AppError;
use crate::utils::jwt::{Claims, create_jwt};

pub async fn create_user_service(ruser: ReqCreateUser) -> Result<u32> {
    let user = User {
        phone: ruser.phone,
        password: md5(&ruser.password, "123")
    };

    let user_info = UserInfo {
        nickname: ruser.nickname,
        avatar: ruser.avatar,
        gender: ruser.gender,
        birthday: ruser.birthday
    };

    let uid= create_user_model((user, user_info)).await?;
    Ok(uid)
}

pub async fn login_service(login: ReqLogin) -> Result<String> {

    let user = User {
        phone: login.phone,
        password: md5(&login.password, "123")
    };

    let res = find(user).await?;

    if res.is_none() {
        return Err(Error::from(AppError::new("没找到用户信息")));
    }

    let user = res.unwrap();
    //创建jwt
    let now = Local::now();

    let c = Claims {
        uid: user.uid,
        exp: (now + Duration::hours(24)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    let token = create_jwt(c).unwrap();
    Ok(token)
}