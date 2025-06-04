// use crypto::digest::Digest;
// use crypto::md5::Md5;
use md5::{Md5, Digest};

// 比较老旧 新版rust不支持 换成了 md5库
// pub fn md5(value: &str, salt: &str) -> String {
//     let mut hash = Md5::new();
//     let str = format!("{}{}", value, salt);
//     hash.input_str(&str);
//     hash.result_str()
// }

pub fn md5(value: &str, salt: &str) -> String {
    let input = format!("{}{}", value, salt);

    // 创建 Md5 hasher 实例
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());

    // 获取哈希结果并转为十六进制字符串
    let result = hasher.finalize();
    format!("{:x}", result)
}