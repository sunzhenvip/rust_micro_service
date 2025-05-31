use axum::{Router};
use axum::routing::{get, post};
use crate::handler::user::{create_user, get_user, login};
use crate::handler::user::get_users;

pub fn user_routers() -> Router {
     Router::new()
         .route("/users/:uid?", get(get_user))
         .route("/users", get( get_users ).post( create_user ))
         .route("/login", post( login ))
}