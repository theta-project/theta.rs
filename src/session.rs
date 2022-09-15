use crate::buf;
use crate::globals;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Session {
    pub id: u32,
    pub username: String,
    pub token: String,
    pub buffer: buf::Buffer,
}

//pub static SESSION_LIST: Lazy<>> = Lazy::new(|| Mutex::new(vec![]));