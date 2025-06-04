use axum::response::{IntoResponse, Response};
use validator::ValidationErrors;
use crate::utils::result::fail_null;

pub struct ErrorWarp(pub ValidationErrors);

impl IntoResponse for ErrorWarp {
    fn into_response(self) -> Response {
        let hm = self.0.field_errors();
        let mut code = 1;
        for (_, v) in hm {
            code = v.get(0).unwrap().code.parse::<usize>().unwrap();
            break;
        }
        fail_null(code)
    }
}