use crate::buf;

#[derive(Debug)]
pub struct Session {
    pub id: u32,
    pub username: String,
    pub token: String,
    pub buffer: buf::Buffer,
}
