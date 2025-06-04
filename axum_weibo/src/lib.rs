#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
// 以上是消除部分编译警告

use crate::config::config_init;
use crate::model::db_conn_init;
use crate::router::start_route;

pub mod config;
// cargo expand --lib entities::wb_user > expanded.rs  生成的派生宏代码还源成源码 保存到 expanded.rs 文件中
pub mod entities;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod router;
pub mod service;
pub mod utils;
// 测试使用的
pub mod test;

pub mod helloworld;

pub async fn run() {
    //初始化
    config_init().await;

    //链接数据库
    db_conn_init().await;

    //开启路由
    start_route().await;
}

#[cfg(test)]
mod test_expand {
    use super::entities::wb_user;

    #[test]
    fn dummy() {
        let _ = wb_user::Model {
            uid: 1,
            phone: "".to_string(),
            password: "".to_string(),
            created_time: 0,
            updated_time: 0,
        };
    }
}


// mod test_expand 这个代码可以执行测试

// 我在当前项目目录下运行了 cargo expand --lib entities::wb_user > expanded.rs 但是 expanded.rs 文件 IDE提示 缺少模块声明。这可能会影响智能编辑功能和自动补全。
// 文件在 /data/network/rust/rust_web_axum/expanded.rs 位置
// 怎么能让 rustrover IDE 能正常提示?

// expanded.rs 文件ide正常提示
// cargo expand --lib entities::wb_user > src/expanded.rs
/*#[cfg(debug_assertions)]
#[path = "expanded.rs"]
mod expanded;
*/