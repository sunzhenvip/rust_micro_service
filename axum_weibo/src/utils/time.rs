use chrono::Local;

pub fn local_timestamp() -> u32 {
    Local::now().timestamp() as u32
}