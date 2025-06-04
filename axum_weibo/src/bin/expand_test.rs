use chrono::Local;
use sea_orm::ActiveValue::Set;
// src/bin/expand_test.rs
use axum_weibo::entities::wb_user;

// cargo expand --bin expand_test 运行这个命令可以查看

fn main() {
    let _ = wb_user::ActiveModel {
        uid: Set(3),
        phone: Set("12333".to_string()),
        password: Set("88888".to_string()),
        created_time: Set(Local::now().timestamp() as u32),
        updated_time: Set(Local::now().timestamp() as u32),
    };
}