use bytes::{BytesMut, BufMut};

pub struct Buffer {
    pub buffer: BytesMut,

    pub length: usize,
    pub offset: usize,
}

pub struct BanchoLogin {
    pub id: u32,
}

impl BanchoLogin {
    pub fn write_to_buffer(&self, buf: &mut Buffer) {
        // push the data
        buf.buffer.put_u16_le(5); // packet id
        buf.buffer.put_u8(0); // empty 'compression' bit
        buf.buffer.put_u32_le(4); // length
        buf.buffer.put_u32_le(self.id);
    }
}
