use bytes::{Buf, BufMut, BytesMut};
#[derive(Debug)]
pub struct Buffer {
    pub buffer: BytesMut,
}

impl Buffer {
    fn put_header(&mut self, id: i16) {
        self.write_i16(id);
        self.write_bool(false);
        self.write_u32(0);
    }

    fn fix_header(&mut self, start: usize) {
        let length = self.buffer.len() - start - 7;
        (self.buffer[start + 3]) = length as u8;
    }

    pub fn write_bool(&mut self, val: bool) {
        self.buffer.put_u8(val as u8);
    }

    pub fn write_u8(&mut self, val: u8) {
        self.buffer.put_u8(val);
    }

    pub fn write_u16(&mut self, val: u16) {
        self.buffer.put_u16_le(val);
    }

    pub fn write_u32(&mut self, val: u32) {
        self.buffer.put_u32_le(val);
    }

    pub fn write_u64(&mut self, val: u64) {
        self.buffer.put_u64_le(val);
    }

    pub fn write_i8(&mut self, val: i8) {
        self.buffer.put_i8(val);
    }

    pub fn write_i16(&mut self, val: i16) {
        self.buffer.put_i16_le(val);
    }

    pub fn write_i32(&mut self, val: i32) {
        self.buffer.put_i32_le(val);
    }

    pub fn write_i64(&mut self, val: i64) {
        self.buffer.put_i64_le(val);
    }

    pub fn write_float(&mut self, val: f32) {
        self.buffer.put_f32_le(val);
    }

    pub fn write_double(&mut self, val: f64) {
        self.buffer.put_f64_le(val);
    }

    fn write_uleb(&mut self, mut len: usize) {
        let mut uleb: Vec<u8> = vec![0; 32];

        let mut uleb_len: usize = 0;

        while len > 0 {
            uleb[uleb_len] = (len as u8) & 0x7f;

            len >>= 7;
            if len != 0 {
                uleb[uleb_len] |= 0x80;
            }

            uleb_len += 1;
        }

        uleb.retain(|&x| x != 0);

        self.buffer.put(uleb.as_slice());
    }
    pub fn write_string(&mut self, string: String) {
        let length = string.len();

        self.write_u8(0xb);
        self.write_uleb(length);
        self.buffer.put(string.as_bytes());
    }

    pub fn read_bool(&mut self) -> bool {
        self.buffer.get_u8() != 0
    }

    pub fn read_u8(&mut self) -> u8 {
        self.buffer.get_u8()
    }

    pub fn read_u16(&mut self) -> u16 {
        self.buffer.get_u16_le()
    }

    pub fn read_u32(&mut self) -> u32 {
        self.buffer.get_u32_le()
    }

    pub fn read_u64(&mut self) -> u64 {
        self.buffer.get_u64_le()
    }

    pub fn read_i8(&mut self) -> i8 {
        self.buffer.get_i8()
    }

    pub fn read_i16(&mut self) -> i16 {
        self.buffer.get_i16_le()
    }

    pub fn read_i32(&mut self) -> i32 {
        self.buffer.get_i32_le()
    }

    pub fn read_i64(&mut self) -> i64 {
        self.buffer.get_i64_le()
    }

    pub fn read_float(&mut self) -> f32 {
        self.buffer.get_f32_le()
    }

    pub fn read_double(&mut self) -> f64 {
        self.buffer.get_f64_le()
    }

    fn read_uleb(&mut self) -> usize {
        let mut result = 0;
        let mut shift = 0;

        let mut byte = self.read_u8();

        if (byte & 0x80) == 0 {
            result |= (byte & 0x7f) << shift;
        } else {
            let mut end = false;

            while !end {
                if shift > 0 {
                    byte = self.read_u8();
                }

                result |= (byte & 0x7f) << shift;
                if (byte & 0x80) == 0 {
                    end = true;
                }
                shift += 7;
            }
        }

        result as usize
    }

    pub fn read_str(&mut self) -> String {
        let marker = self.read_u8();
        let length = self.read_uleb();
        let mut string = "".to_string();

        if length > 0 {
            let mut i = 0;

            while i < length + 1 {
                let current_char = self.buffer.get(i);
                let chr = match current_char {
                    Some(x) => *x as char,
                    None => '\0',
                };

                string = format!("{}{}", string, chr); //;
                i += 1;
            }

            self.buffer.advance(length);
        }

        string
    }

    pub fn packet_login_success(&mut self, id: u32) {
        let start = self.buffer.len();

        self.put_header(5);
        self.write_u32(id);
        self.fix_header(start);
    }

    pub fn packet_announce(&mut self, announcement: String) {
        let start = self.buffer.len();

        self.put_header(24);
        self.write_string(announcement);
        self.fix_header(start);
    }

    pub fn packet_channel_join(&mut self, chan: String) {
        let start = self.buffer.len();

        self.put_header(64);
        self.write_string(chan);
        self.fix_header(start);
    }

    pub fn packet_bancho_restart(&mut self) {
        self.put_header(86);
    }
}

/*
impl BanchoLogin {
    pub fn write_to_buffer(&self, buf: &mut Buffer) {
        // push the data
        buf.buffer.put_u16_le(5); // packet id
        buf.buffer.put_u8(0); // empty 'compression' bit
        buf.buffer.put_u32_le(4); // length
        buf.buffer.put_u32_le(self.id);
    }
}*/
