use std::{vec, sync::Mutex};

use once_cell::sync::OnceCell;

use crate::session;

pub struct Globals {
    pub session_list: Mutex<Vec<session::Session>>,
    pub handled_requests: Mutex<u32>
}