use std::fmt::format;
use std::str::Utf8Error;

use actix_web::{web, HttpResponseBuilder};
use bytes::Buf;
use bytes::BytesMut;
use uuid::Uuid;

use crate::buf;
use crate::globals;
use crate::session;

fn parse_login_data(body: &web::BytesMut) -> Result<(String, String, String), &'static str> {
    let parsed_body =
        std::str::from_utf8(body).map_err(|_x: Utf8Error| "Error parsing body data")?;
    let login_data: Vec<&str> = parsed_body.lines().collect();

    let username = login_data.get(0).cloned().ok_or("username not in vector")?;
    let password = login_data.get(1).cloned().ok_or("password not in vector")?;
    let client_extra = login_data
        .get(2)
        .cloned()
        .ok_or("client extra not in vector")?;

    Ok((
        username.to_string(),
        password.to_string(),
        client_extra.to_string(),
    ))
}

pub fn login(
    body: &web::BytesMut,
    res: &mut HttpResponseBuilder,
    globals: web::Data<globals::Globals>,
) -> BytesMut {
    let (username, password, extra) = match parse_login_data(body) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return BytesMut::default();
        }
    };
    let mut global_sessions = globals.session_list.lock().unwrap();

    let sess = session::Session {
        id: 10,
        username,
        token: Uuid::new_v4().to_string(),
        buffer: buf::Buffer {
            buffer: BytesMut::default(),
        },
    };
    global_sessions.push(sess);
    // sess no longer exists here
    let sess_len = global_sessions.len();
    let sess: &mut session::Session = &mut global_sessions[sess_len- 1];

    println!("username: {}", sess.username);
    println!("password: {}", password);
    println!("client extra: {}", extra);

    sess.buffer.packet_login_success(sess.id);
    sess.buffer
        .packet_announce(format!("Welcome to theta, {}!", sess.username));
    sess.buffer.packet_channel_join("#osu".to_string());

    res.insert_header(("cho-token", sess.token.clone()));
    sess.buffer.buffer.clone()
    //
}
//sess: &mut session::Session
pub fn handle_packet(body: &web::BytesMut, globals: web::Data<globals::Globals>) -> BytesMut {
    let in_buf = buf::Buffer {
        buffer: body.clone(),
    };
    in_buf.buffer

    /*     if !sess.buffer.buffer.is_empty() {
         sess.buffer.buffer.clear();
     }

     let mut length = 0;
     while length <= in_buf.buffer.len() {
         let id = in_buf.read_i16();
         let _compression = in_buf.read_bool();
         let packet_length = in_buf.read_u32();


         if id == 0 {
             let status = packet::read_status(&mut in_buf);
             println!("{:?}", status);
         } else {
             println!("Unhandled packet: {} (length: {})", id, packet_length);
             in_buf.buffer.advance(packet_length as usize);
             length += packet_length as usize;
         }



         length += 1;
     }

    .. sess.buffer.buffer.clone()*/
}
