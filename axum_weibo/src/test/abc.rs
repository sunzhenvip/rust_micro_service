// 测试模块导入导出引入问题

// 重新引入 super 表示 test 路径 然后引入 test::wb_user 变相 写 super::wb_user 引入对应的模块
pub use super::wb_user::UserAll as xxx;
