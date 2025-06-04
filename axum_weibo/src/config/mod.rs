use crate::config::app_config::app_init;
use crate::config::status_code::code_init;

pub mod app_config;
pub mod status_code;

pub async fn config_init() {
    app_init().await;
    code_init().await;
}