use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use validator::Validate;
use crate::middleware::jwt::Auth;
use crate::service::feed::feed_service;
use crate::utils::result::{fail_null, success};
use crate::utils::warp::ErrorWarp;

#[derive(Deserialize, Validate)]
pub struct ReqFeed {
    #[validate(range(min = 1, code="17001"))]
    pid: u32,
    #[validate(range(min = 1, max = 50,  code="17002"))]
    size: u8
}

pub async fn feed (auth: Auth, Json(feed): Json<ReqFeed>) -> Response {
    if let Err(e) =  feed.validate(){
        return ErrorWarp(e).into_response()
    }

    let res = feed_service(auth.uid, feed.pid, feed.size).await;
    if res.is_err() {
        return fail_null(1);
    }
    success(res.unwrap())
}