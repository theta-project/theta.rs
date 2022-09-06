use bytes::{BytesMut, BufMut};

pub struct Buffer {
    pub buffer: BytesMut,

    pub length: usize,
    pub offset: usize,
}

impl Buffer {
    pub fn write_bool(&mut self, val: bool) {
        self.buffer.put_u8(val as u8);
        self.length += std::mem::size_of::<u8>();
    }

    pub fn write_u8(&mut self, val: u8) {
        self.buffer.put_u8(val);
        self.length += std::mem::size_of::<u8>();
    }

    pub fn write_u16(&mut self, val: u16) {
        self.buffer.put_u16_le(val);
        self.length += std::mem::size_of::<u16>();
    }

    pub fn write_u32(&mut self, val: u32) {
        self.buffer.put_u32_le(val);
        self.length += std::mem::size_of::<u32>();
    }

    pub fn write_u64(&mut self, val: u64) {
        self.buffer.put_u64_le(val);
        self.length += std::mem::size_of::<u64>();
    }

    pub fn write_i8(&mut self, val: i8) {
        self.buffer.put_i8(val);
        self.length += std::mem::size_of::<i8>();
    }

    pub fn write_i16(&mut self, val: i16) {
        self.buffer.put_i16_le(val);
        self.length += std::mem::size_of::<i16>();
    }

    pub fn write_i32(&mut self, val: i32) {
        self.buffer.put_i32_le(val);
        self.length += std::mem::size_of::<i32>();
    }

    pub fn write_i64(&mut self, val: i64) {
        self.buffer.put_i64_le(val);
        self.length += std::mem::size_of::<i64>();
    }

    pub fn write_float(&mut self, val: f32) {
        self.buffer.put_f32_le(val);
        self.length += std::mem::size_of::<f32>();
    }

    pub fn write_double(&mut self, val: f64) {
        self.buffer.put_f64_le(val);
        self.length += std::mem::size_of::<f64>();
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
        //self.write_u8(uleb_len as u8);
    }
    pub fn write_string(&mut self, string: String) {
        let length = string.len();

        self.write_u8(0xb);
        self.write_uleb(length);
        self.buffer.put(string.as_bytes());
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



