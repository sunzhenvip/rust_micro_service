use moka::sync::Cache;
use tokio::sync::OnceCell;
use crate::service::user::RspUser;

pub static LOCAL_CACHE: OnceCell<Cache<u32, RspUser>> = OnceCell::const_new();
pub async fn local_cache_init() {
    LOCAL_CACHE.get_or_init(|| async {
        let c:Cache<u32, RspUser> = Cache::builder()
            .name("account")
            .max_capacity(10000)
            .time_to_idle(std::time::Duration::from_secs(300))
            .build();
        c
    }).await;
}