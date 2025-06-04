use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::service::user::{create_user_service, login_service};
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