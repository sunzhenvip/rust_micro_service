use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn md5(value: &str, salt: &str) -> String {
    let mut hash = Md5::new();
    let str = format!("{}{}", value, salt);
    hash.input_str(&str);
    hash.result_str()
}