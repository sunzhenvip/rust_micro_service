use opentelemetry::{global, KeyValue};
use opentelemetry::trace::{Span, SpanKind, Tracer};
use tonic::{Code, Request, Response, Status};
use tools::trace::ExtractorMetadataMap;
use crate::proto::account::account_server::Account;
use crate::proto::account::{AccountReply, AccountRequest};
use crate::proto::account::account_reply::User;
use crate::service::user::get_users_by_uids_service;

#[derive(Default)]
pub struct AccountRpc {} //2.自定义结构实现trait

#[tonic::async_trait]
impl Account for AccountRpc {
    async fn get_users_by_uids(&self, request: Request<AccountRequest>) -> Result<Response<AccountReply>, Status> {

        let uids = request.get_ref().uid.clone();
        let users = get_users_by_uids_service(uids).await;
        if users.is_err() {
            return Err(Status::new(Code::Unavailable, "服务不可用"));
        }


        let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&ExtractorMetadataMap(request.metadata())));
        let tracer = global::tracer("feed-app");
        let mut span = tracer
            .span_builder("Greeter/server")
            .with_kind(SpanKind::Server)
            .start_with_context(&tracer, &parent_cx);
        span.add_event("users", vec![KeyValue::new("uids", "uids")]);

        let users: Vec<User> = users.unwrap().into_iter().map(|x| {
            let avatar = x.avatar.unwrap_or("".to_string());
            let nickname = x.nickname.unwrap_or("".to_string());
            User{
                uid: x.uid,
                level: x.level as u32,
                follow_count: x.follow_count as u32,
                fans_count: x.fans_count,
                nickname,
                avatar,
            }
        }).collect();

        let rsp = AccountReply{
            users,
        };

        Ok(Response::new(rsp))
    }
}