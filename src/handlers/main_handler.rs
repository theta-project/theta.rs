use std::{str::Utf8Error};

use actix_web::{web, HttpResponseBuilder};
use bytes::{BytesMut};

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
    
    let login_packet = buf::BanchoLogin {
        id: 300000
    };

    login_packet.write_to_buffer(&mut buf);

    println!("{:?}", buf.buffer);

    res.insert_header(("cho-token", username));
    buf.buffer
}
/*
pub fn handle_packet(req: HttpRequest, res: HttpResponseBuilder) {

}*/
