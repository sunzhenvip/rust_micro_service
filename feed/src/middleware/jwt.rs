use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::{header, StatusCode};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use crate::utils::jwt::check_token;
use crate::utils::result::fail_null;

pub struct Auth {
    pub uid: u32,
    pub level: u8,
    pub is_super: u8,
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
        
        let auth_header: Option<&str> = auth_header;
        if auth_header.is_none() {
            return Err(StatusCode::UNAUTHORIZED.into_response());
        }

        let token = auth_header.unwrap();
        let claims = check_token(token);

        if let Ok(token_data) = claims {
            let uid = token_data.claims.uid;
            let level = token_data.claims.level;
            let is_super = token_data.claims.is_super;
            Ok(Auth{ uid, level, is_super})
        } else {
            Err(fail_null(2))
        }
    }
}