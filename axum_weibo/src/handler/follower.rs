use axum::extract::Path;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use validator::Validate;
use crate::middleware::jwt::Auth;
use crate::service::follower::{follow_service, unfollow_service};
use crate::utils::result::{fail_null, success_null};
use crate::utils::warp::ErrorWarp;

#[derive(Deserialize, Validate)]
pub struct ReqCreateFollower {
    #[validate(range(min = 1, code="15002"))]
    followee: u32
}

pub async fn follow (auth: Auth, Json(user): Json<ReqCreateFollower>) -> Response {
    if let Err(e) =  user.validate(){
        return ErrorWarp(e).into_response()
    }

    if auth.uid == user.followee {
        return fail_null(15001);
    }

    let res = follow_service(auth.uid, user.followee).await;

    if res.is_err() {
        return fail_null(1);
    }
    success_null()
}

pub async fn unfollow (auth: Auth, Path(followee): Path<u32>) -> Response {

    if auth.uid == followee {
        return fail_null(15001);
    }

    let res = unfollow_service(auth.uid, followee).await;
    if res.is_err() {
        return fail_null(1);
    }
    success_null()
}