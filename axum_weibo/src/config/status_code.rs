use std::collections::HashMap;
use tokio::sync::OnceCell;

pub static STATUS_CODE: OnceCell<HashMap<usize, &str>> = OnceCell::const_new();


pub async fn code_init() {
    STATUS_CODE.get_or_init(|| async {
         HashMap::from([
             (0, "操作成功"),
             (1, "操作失败"),
             (2, "jwt验证失败"),

             //10_000 - 12_000 用户
             (10001, "手机号格式不正确"),
             (10002, "密码长度不正确"),
             (10003, "账号或者密码不正确"),

             //12_001 - 13_000 微博
             (12001, "内容长度不正确"),

             //13_001 - 15_000 评论


             //15_001 - 16_000 关注
             (15001, "不能关注或取关自己"),
             (15002, "被关注uid不正确"),



             //17_001 - 18_000
             (17001, "必须传入feed_id"),
             (17002, "size 参数范围不正确"),
        ])
    }).await;
}