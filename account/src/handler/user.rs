use std::collections::HashMap;
use axum::extract::{Path, Query};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::service::user::{create_user_service, get_users_by_uids_service, login_service};
use crate::utils::result::{fail_null, success};
use crate::utils::warp::ErrorWarp;

#[derive(Debug, Deserialize, Validate)]
pub struct ReqCreateUser {
    #[validate(custom(function="crate::utils::validate::validate_phone", code="10001"))]
    pub phone: String,
    #[validate(length(min=6, max=20, code="10002"))]
    pub password: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub gender: Option<u8>,
    pub birthday: Option<u32>
}

#[derive(Serialize)]
pub struct RespUser {
    pub uid: u32
}

pub async fn create_user(Json(user): Json<ReqCreateUser>) -> Response {

    //验证
    if let Err(e) = user.validate() {
        return ErrorWarp(e).into_response();
    }

    println!("{:?}", user);
    //调用service
    if let Ok(uid) = create_user_service(user).await {
        return success(RespUser{
            uid
        });
    }
    fail_null(1)
}

#[derive(Deserialize, Validate)]
pub struct ReqLogin {
    #[validate(custom(function="crate::utils::validate::validate_phone", code="10001"))]
    pub phone: String,
    #[validate(length(min=6, max=20, code="10002"))]
    pub password: String
}

#[derive(Serialize)]
struct RespJwt {
    token: String
}

pub async fn login(Json(login): Json<ReqLogin>) -> Response {
    //验证
    if let Err(e) = login.validate() {
        return ErrorWarp(e).into_response();
    }

    if let Ok(token) = login_service(login).await {
        return success(RespJwt {
            token
        });
    }
    fail_null(10003)
}

pub async fn get_users(Query(q): Query<HashMap<String, String>>) -> Response {

    if q.is_empty() {
        return fail_null(10004);
    }

    let q = q.get("uids");

    //验证
    if q.is_none() {
        return fail_null(10004);
    }

    let uids: Vec<u32> = q.unwrap().split(",").map(|x| {
        x.parse().unwrap()
    }).collect();

    //调用service
    if let Ok(users) =  get_users_by_uids_service(uids).await{
        if !users.is_empty() {
            return success(users);
        }
    }
    fail_null(1)
}

pub async fn get_user(Path(uid): Path<u32>) -> Response {
    //验证
    if uid == 0 {
        return fail_null(10004);
    }

    let uids = vec![uid];
    //调用service
    if let Ok(users) =  get_users_by_uids_service(uids).await{
        let user = users.first().unwrap();
        return success(user);
    }
    fail_null(1)
}