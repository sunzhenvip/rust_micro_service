pub mod user;
pub mod follower;
pub mod feed;
pub mod post;

use sea_orm::{Database, DbConn};
use tokio::sync::OnceCell;
use crate::config::app_config::APP_CONFIG;

pub static DB_CONN: OnceCell<DbConn> = OnceCell::const_new();

pub async fn db_conn_init() {
    DB_CONN.get_or_init(|| async {
        let db = &APP_CONFIG.get().unwrap().db;
        let db_url = format!("mysql://{}:{}@{}:{}/{}",
                             db.username,
                             db.password,
                             db.host,
                             db.port,
                             db.database);
        let db = Database::connect(db_url).await.expect("数据库链接失败");
        db
    }).await;
}

pub fn get_db() -> &'static DbConn {
    let db = DB_CONN.get().unwrap();
    db
}

