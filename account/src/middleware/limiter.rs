use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::{StatusCode};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use sentinel_core::{base, EntryBuilder};

pub struct Limiter;

#[async_trait]
impl<S> FromRequestParts<S> for Limiter
    where
        S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {

        let entry_builder = EntryBuilder::new(String::from("account-limiter")).with_traffic_type(base::TrafficType::Inbound);

        //4.检查是否能通过规则
        if let Ok(entry) = entry_builder.build() {

            entry.exit();
            return Ok(Limiter)

        }
        return Err(StatusCode::FORBIDDEN.into_response())
    }
}