use std::mem::transmute;
use tracing::debug;

#[derive(Debug)]
pub struct Buffer {
    pub position: usize,
    pub array: Vec<u8>,
}

impl Buffer {
    pub fn write_i8(&mut self, i: i8) {
        let mut bytes: [u8; 1] = unsafe { transmute(i.to_be()) };
        for x in bytes {
            self.write_u8(x);
        }
    }

    pub fn write_i8_le(&mut self, i: i8) {
        let mut bytes: [u8; 1] = unsafe { transmute(i.to_le()) };
        for x in bytes {
            self.write_u8(x);
        }
    }

    pub fn write_u8(&mut self, i: u8) {
        let position = self.position;
        self.position += 1;
        self.array.insert(position, i);
    }

    pub fn write_u8_le(&mut self, i: u8) {
        let mut bytes: [u8; 1] = unsafe { transmute(i.to_le()) };
        for x in bytes {
            self.write_u8(x);
        }
    }
    //
    pub fn write_i16(&mut self, i: i16) {
        let mut bytes: [u8; 2] = unsafe { transmute(i.to_be()) };
        for x in bytes {
            self.write_u8(x);
        }
    }

    pub fn write_i16_le(&mut self, i: i16) {
        let mut bytes: [u8; 2] = unsafe { transmute(i.to_le()) };
        for x in bytes {
            self.write_u8(x);
        }
    }

    pub fn write_u16(&mut self, i: u16) {
        let mut bytes: [u8; 2] = unsafe { transmute(i.to_be()) };
        for x in bytes {
            self.write_u8(x);
        }
    }

    pub fn write_u16_le(&mut self, i: u16) {
        let mut bytes: [u8; 2] = unsafe { transmute(i.to_le()) };
        for x in bytes {
            self.write_u8(x);
        }
    }
    //
    pub fn write_i32(&mut self, i: i32) {
        let mut bytes: [u8; 4] = unsafe { transmute(i.to_be()) };
        for x in bytes {
            self.write_u8(x);
        }
    }

    pub fn write_i32_le(&mut self, i: i32) {
        let mut bytes: [u8; 4] = unsafe { transmute(i.to_le()) };
        for x in bytes {
            self.write_u8(x);
        }
    }

    pub fn write_u32(&mut self, i: u32) {
        let mut bytes: [u8; 4] = unsafe { transmute(i.to_be()) };
        for x in bytes {
            self.write_u8(x);
        }
    }

    pub fn write_u32_le(&mut self, i: u32) {
        let mut bytes: [u8; 4] = unsafe { transmute(i.to_le()) };
        for x in bytes {
            self.write_u8(x);
        }
    }

    pub fn write_packed_u32(&mut self, i: u32) {
        let mut value = i;
        loop {
            let mut b = (i & 0xFF) as u8;
            if value >= 0x80 {
                b |= 0x80;
            }
            self.write_u8(b);
            value >>= 7;
            if value <= 0 {
                break;
            }
        }
    }

    pub fn write_string(&mut self, str: String) {
        let bytes = str.as_bytes();
        self.write_packed_u32(bytes.len() as u32);
        for x in bytes {
            self.write_u8(*x);
        }
    }

    //

    pub fn read_i8(&mut self) -> i8 {
        let position = self.position;
        self.position += 1;
        let array: &[u8] = &self.array[position..self.position];
        return i8::from_be_bytes(array.try_into().unwrap());
    }

    pub fn read_u8(&mut self) -> u8 {
        let position = self.position;
        self.position += 1;
        let array: &[u8] = &self.array[position..self.position];
        return u8::from_be_bytes(array.try_into().unwrap());
    }

    pub fn read_i16(&mut self) -> i16 {
        let position = self.position;
        self.position += 2;
        let array: &[u8] = &self.array[position..self.position];
        return i16::from_be_bytes(array.try_into().unwrap());
    }

    pub fn read_u16(&mut self) -> u16 {
        let position = self.position;
        self.position += 2;
        let array: &[u8] = &self.array[position..self.position];
        return u16::from_be_bytes(array.try_into().unwrap());
    }

    pub fn read_i32(&mut self) -> i32 {
        let position = self.position;
        self.position += 4;
        let array: &[u8] = &self.array[position..self.position];
        let mut empty_bytes: [u8; 4] = [0; 4];
        for i in 0..array.len() {
            empty_bytes[i] = array[i];
        }
        unsafe { transmute::<[u8; 4], i32>(empty_bytes) }
    }
    pub fn read_u32(&mut self) -> u32 {
        let position = self.position;
        self.position += 4;
        let array = &self.array[position..self.position];
        let mut empty_bytes: [u8; 4] = [0; 4];
        for i in 0..array.len() {
            empty_bytes[i] = array[i];
        }
        unsafe { transmute::<[u8; 4], u32>(empty_bytes) }
    }

    pub fn read_f32(&mut self) -> f32 {
        let position = self.position;
        self.position += 4;
        let array: &[u8] = &self.array[position..self.position];
        return f32::from_le_bytes(array.try_into().unwrap());
    }

    pub fn read_i64(&mut self) -> i64 {
        let position = self.position;
        self.position += 8;
        let array: &[u8] = &self.array[position..self.position];
        return i64::from_be_bytes(array.try_into().unwrap());
    }

    pub fn read_string(&mut self) -> String {
        let length = self.read_packed_uint_32();
        let position = self.position;
        self.position += length as usize;
        let array: &[u8] = &self.array[position..self.position];
        return String::from_utf8(array.try_into().unwrap()).unwrap();
    }

    pub fn read_bool(&mut self) -> bool {
        let byte = self.read_i8();
        return byte != 0;
    }

    pub fn read_packed_uint_32(&mut self) -> u32 {
        let mut read_more = true;
        let mut output: u32 = 0;
        let mut shift = 0;
        while read_more {
            let mut byte: u8 = self.read_u8();
            if byte >= 0x80 {
                read_more = true;
                byte ^= 0x80;
            } else {
                read_more = false;
            }

            output |= byte.checked_shl(shift).unwrap_or(0) as u32;
            shift += 7;
        }
        return output;
    }

    pub fn new(buff: &mut Buffer) -> Buffer {
        let array = &*buff.array;
        let pos = buff.position;
        Buffer {
            array: Vec::from(array),
            position: pos,
        }
    }
}
