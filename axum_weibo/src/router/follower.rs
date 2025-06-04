use axum::Router;
use axum::routing::{delete, post};
use crate::handler::follower::{follow, unfollow};

pub fn follower_routers() -> Router {
    Router::new()
        .route("/followers", post( follow ))
        .route("/followers/:uid", delete(unfollow))
}