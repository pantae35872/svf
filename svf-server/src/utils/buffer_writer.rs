use std::u16;

#[derive(Debug)]
pub struct BufferWriter<'a> {
    buffer: &'a mut Vec<u8>,
}

impl<'a> BufferWriter<'a> {
    pub fn new(buffer: &'a mut Vec<u8>) -> Self {
        Self { buffer }
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> &mut Self {
        self.buffer.extend_from_slice(data);
        self
    }

    pub fn write_string(&mut self, string: String) -> &mut Self {
        let str_byte = string.as_bytes();
        self.write_u32(str_byte.len() as u32);
        self.write_bytes(str_byte);
        self
    }

    pub fn write_i64(&mut self, data: i64) -> &mut Self {
        self.write_bytes(&data.to_le_bytes());
        self
    }

    pub fn write_i32(&mut self, data: i32) -> &mut Self {
        self.write_bytes(&data.to_le_bytes());
        self
    }

    pub fn write_i16(&mut self, data: i16) -> &mut Self {
        self.write_bytes(&data.to_le_bytes());
        self
    }

    pub fn write_i8(&mut self, data: i8) -> &mut Self {
        self.write_bytes(&data.to_le_bytes());
        self
    }

    pub fn write_u64(&mut self, data: u64) -> &mut Self {
        self.write_bytes(&data.to_le_bytes());
        self
    }

    pub fn write_u32(&mut self, data: u32) -> &mut Self {
        self.write_bytes(&data.to_le_bytes());
        self
    }

    pub fn write_u16(&mut self, data: u16) -> &mut Self {
        self.write_bytes(&data.to_le_bytes());
        self
    }

    pub fn write_u8(&mut self, data: u8) -> &mut Self {
        self.write_bytes(&data.to_le_bytes());
        self
    }

    pub fn write_bool(&mut self, data: bool) -> &mut Self {
        self.write_u8(if data { 1 } else { 0 });
        self
    }
}
