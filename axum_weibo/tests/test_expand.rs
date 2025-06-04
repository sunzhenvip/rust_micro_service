use axum_weibo::entities::wb_user;

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

// cargo expand --lib entities::wb_user > expanded.rs 这种确实可以
// 但是我还是想尝试 cargo expand --test test_expand
// 然后又出错了
// sz:/data/network/rust/rust_web_axum$ cargo expand --test test_expand
// error: no test target named `test_expand`.
