use std::str::Utf8Error;

use actix_web::{web, HttpResponseBuilder};
use bytes::BytesMut;

use crate::buf;

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

pub fn login(body: &web::BytesMut, res: &mut HttpResponseBuilder) -> BytesMut {
    let (username, password, extra) = match parse_login_data(body) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return BytesMut::default();
        }
    };

    println!("username: {}", username);
    println!("password: {}", password);
    println!("client extra: {}", extra);

    let mut buf = buf::Buffer {
        buffer: BytesMut::default(),
        length: 0,
        offset: 0,
    };

    buf.write_i16(5);
    buf.write_bool(false);
    buf.write_u32(4);
    buf.write_u32(10);

    let announcement = "nya~";
    buf.write_i16(24);
    buf.write_bool(false);
    buf.write_u32((announcement.len() as u32) + 2);
    buf.write_string(announcement.to_string());

    buf.write_i16(64);
    buf.write_bool(false);
    buf.write_u32(("#osu".len() as u32) + 2);
    buf.write_string("#osu".to_string());

    println!("{:?}", buf.buffer);


    let mut buf_test = buf::Buffer {
        buffer: BytesMut::default(),
        length: 0,
        offset: 0,
    };

    buf_test.write_i16(5);
    buf_test.write_bool(false);
    buf_test.write_u32((announcement.len() as u32) + 2);
    buf_test.write_string(announcement.to_string());

    let packet_id = buf_test.read_i16();
    println!("{}", packet_id);

    let _compression = buf_test.read_bool();
    let length = buf_test.read_u32();
    let userid = buf_test.read_str();
    println!("{}", length);
    println!("{}", userid);

    res.insert_header(("cho-token", username));
    buf.buffer
}
/*
pub fn handle_packet(req: HttpRequest, res: HttpResponseBuilder) {

}*/
