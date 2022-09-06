use crate::buf;

fn put_header(b: &mut buf::Buffer, id: i16) {
    b.write_i16(id);
    b.write_bool(false);
    b.write_u32(0);
}
fn fix_header(b: &mut buf::Buffer, start: usize) {
    let length = b.buffer.len() - start - 7;
    (b.buffer[start + 3]) = length as u8;
}

pub fn packet_login_success(b: &mut buf::Buffer, id: u32) {
    let start = b.buffer.len();

    put_header(b, 5);
    b.write_u32(id);
    fix_header(b, start);
}

pub fn packet_announce(b: &mut buf::Buffer, announcement: String) {
    let start = b.buffer.len();

    put_header(b, 24);
    b.write_string(announcement);
    fix_header(b, start);
}


pub fn packet_channel_join(b: &mut buf::Buffer, chan: String) {
    let start = b.buffer.len();

    put_header(b, 64);
    b.write_string(chan);
    fix_header(b, start);
}

#[derive(Debug)]
pub struct ClientStatus {
    pub status: u8,
    pub status_text: String,
    pub beatmap_checksum: String,
    pub current_mods: u32,
    pub play_mode: u8,
    pub beatmap_id: i32,
}

pub fn read_status(b: &mut buf::Buffer) -> ClientStatus {
    ClientStatus {
        status: b.read_u8(),
        status_text: b.read_str(),
        beatmap_checksum: b.read_str(),
        current_mods: b.read_u32(),
        play_mode: b.read_u8(),
        beatmap_id: b.read_i32(),
    }
}
