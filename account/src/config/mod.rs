use crate::config::app_config::app_init;
use crate::config::local_cache::local_cache_init;
use crate::config::sentinel::sentinel_init;
use crate::config::status_code::code_init;

pub mod app_config;
pub mod status_code;
pub mod local_cache;
pub mod sentinel;

pub async fn config_init() {
    app_init().await;
    code_init().await;
    local_cache_init().await;
    sentinel_init();
}