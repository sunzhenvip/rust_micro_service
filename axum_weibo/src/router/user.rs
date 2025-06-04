use axum::Router;
use axum::routing::{post};
use crate::handler::user::{create_user, login};

pub fn user_routers() -> Router {
     Router::new()
         .route("/users", post( create_user ))
         .route("/login", post( login ))
}