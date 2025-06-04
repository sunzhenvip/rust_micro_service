use std::fs;
use serde::Deserialize;
use tokio::sync::OnceCell;

pub static APP_CONFIG: OnceCell<AppConfig> = OnceCell::const_new();


pub async fn app_init() {
    APP_CONFIG.get_or_init(|| async {

        let file = "app.toml";

        //读取配置文件里面的信息
        let content = fs::read_to_string(file).expect("读取配置失败");

        toml::from_str::<AppConfig>(&content).expect("映射配置失败")

    }).await;
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub db: DbConfig,
    pub cache: CacheConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String
}

#[derive(Debug, Deserialize)]
pub struct CacheConfig {
    pub host: String,
    pub port: u16
}

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    pub secret: String
}