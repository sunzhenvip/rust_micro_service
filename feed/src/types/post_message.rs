#[derive(Debug, Clone, Copy)]
pub struct PostMessage {
    pub uid: u32,
    pub pid: u64,
    pub level: u8,
    pub created_time: u32,
}

impl PostMessage {
    pub fn to_bytes(&self) -> [u8; std::mem::size_of::<PostMessage>()] {
        unsafe {
            std::mem::transmute(*self)
        }
    }

    pub fn from_bytes(bytes: [u8; std::mem::size_of::<PostMessage>()]) -> PostMessage {
        unsafe {
            std::mem::transmute(bytes)
        }
    }
}