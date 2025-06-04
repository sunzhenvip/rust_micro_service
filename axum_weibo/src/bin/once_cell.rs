#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数
#![allow(non_upper_case_globals)] // 消除static变量的命名建议使用全大写

// 引入依赖库
use tokio::sync::OnceCell;

#[tokio::main]
async fn main() {
    static CELL: OnceCell<String> = OnceCell::const_new();

    let value = CELL
        .get_or_init(|| async {
            // 异步初始化逻辑
            "Hello, World!".to_string()
        })
        .await;
    println!("{}", value); // 输出: "Hello, World!"
    static CEll1: OnceCell<String> = OnceCell::const_new();

    let val1 = CELL
        .get_or_init(|| async { "Hello, World!".to_string() })
        .await;

    println!("{}", val1); // 输出: "Hello, World!"
}
