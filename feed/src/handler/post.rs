use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use validator::Validate;
use crate::middleware::jwt::Auth;
use crate::service::post::{create_post_service};
use crate::utils::result::{fail_null, success_null};
use crate::utils::warp::ErrorWarp;

#[derive(Deserialize, Validate)]
pub struct ReqCreatePost {
    #[validate(length(min = 1, max=140, code="12001"))]
    content: String
}

pub async fn create_post (auth: Auth, Json(post): Json<ReqCreatePost>) -> Response {
    if let Err(e) =  post.validate(){
        return ErrorWarp(e).into_response()
    }

    //发布动态
    let res = create_post_service(auth.uid, auth.level,  post.content).await;

    if res.is_err() {
        return fail_null(1);
    }
    success_null()
}