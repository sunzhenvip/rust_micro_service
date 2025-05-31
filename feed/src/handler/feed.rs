use std::collections::HashMap;
use axum::extract::{Query};
use axum::response::{Response};
use crate::middleware::jwt::Auth;
use crate::service::feed::get_feeds_service;
use crate::utils::result::{fail_null, success};

pub async fn get_feeds(auth: Auth, Query(time): Query<HashMap<String, u32>>) -> Response {
    let time = *(time.get("time").unwrap());
    let Auth{uid, level:_, is_super} = auth;

    let feeds = get_feeds_service(uid, is_super, time).await;

    if feeds.is_err() {
        return fail_null(1);
    }
    success(feeds.unwrap())
}