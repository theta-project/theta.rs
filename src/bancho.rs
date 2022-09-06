use std::str::Utf8Error;

use actix_web::{web, HttpResponseBuilder};
use bytes::Buf;
use bytes::BytesMut;

use crate::buf;
use crate::packet;

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
        buffer: BytesMut::default()
    };

    buf.write_i16(packet::PacketIDs::BanchoLoginReply as i16);
    buf.write_bool(false);
    buf.write_u32(4);
    buf.write_u32(10);

    let announcement = "nya~";
    buf.write_i16(packet::PacketIDs::BanchoAnnounce as i16);
    buf.write_bool(false);
    buf.write_u32((announcement.len() as u32) + 2);
    buf.write_string(announcement.to_string());

    buf.write_i16(packet::PacketIDs::BanchoChannelJoinSuccess as i16);
    buf.write_bool(false);
    buf.write_u32(("#osu".len() as u32) + 2);
    buf.write_string("#osu".to_string());


    res.insert_header(("cho-token", username));
    buf.buffer
}

pub fn handle_packet(body: &web::BytesMut)  {
    let mut in_buf = buf::Buffer {
        buffer: body.clone()
    };

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

}
