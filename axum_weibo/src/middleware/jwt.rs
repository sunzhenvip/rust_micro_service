use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::{header, StatusCode};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use crate::utils::jwt::check_token;
use crate::utils::result::fail_null;

pub struct Auth {
    pub uid: u32
}

#[async_trait]
impl<S> FromRequestParts<S> for Auth
    where
        S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());

        //没传jwt
        if let None = auth_header {
            return Err(StatusCode::UNAUTHORIZED.into_response());
        }

        let auth_header: Vec<&str> = auth_header.unwrap().split(" ").collect();
        let token = auth_header.get(1);

        if let None = token {
            return Err(StatusCode::UNAUTHORIZED.into_response());
        }

        let token = token.unwrap();
        let claims = check_token(token);

        if let Ok(token_data) = claims {
            let uid = token_data.claims.uid;
            Ok(Auth{ uid})
        } else {
            Err(fail_null(2))
        }
    }
}