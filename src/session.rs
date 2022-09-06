use crate::buf;
#[derive(Debug)]
pub struct Session {
    pub id: u32,
    pub username: String,
    pub token: String,
    pub buffer: buf::Buffer
}


pub const ONLINE_SESSIONS: Vec<&mut Session> = vec![];

pub fn find_from_token(token: String) -> Result<&'static mut Session, String> {
    for session in ONLINE_SESSIONS {
        println!("{} <-> {}", session.token, token);
        if session.token == token {
            println!("sess {:?}", session);
            return Ok(session);
        }
    }
    
    Err("fail".to_string())
}