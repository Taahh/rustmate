use byteorder::{BigEndian, ReadBytesExt};
use std::mem::transmute;

#[derive(Debug, Clone)]
pub struct Buffer {
    position: usize,
    byte_array: Vec<u8>,
}

impl Buffer {
    pub fn new(array: Vec<u8>) -> Self {
        return Buffer {
            position: 0,
            byte_array: array,
        };
    }
    pub fn read_byte(&mut self) -> i8 {
        let position = self.position.clone();
        self.position += 1;

        let vec = &mut self.byte_array.clone();

        let x = &vec[position..self.position];
        return x[0] as i8;
    }

    pub fn read_unsigned_byte(&mut self) -> u8 {
        let position = self.position.clone();
        self.position += 1;

        let vec = &mut self.byte_array.clone();

        let x = &vec[position..self.position];
        let mut empty_bytes: [u8; 1] = [0; 1];
        for i in 0..x.len() {
            empty_bytes[i] = x[i];
        }
        let float = unsafe { transmute::<[u8; 1], u8>(empty_bytes) };
        return float;
    }

    pub fn read_uint_16(&mut self) -> u16 {
        let position = self.position.clone();
        self.position += 2;

        let vec = &mut self.byte_array.clone();

        let x = &vec[position..self.position];

        return x.clone().read_u16::<BigEndian>().unwrap();
    }

    pub fn read_int_32(&mut self) -> i32 {
        let position = self.position.clone();
        self.position += 4;

        let vec = &mut self.byte_array.clone();

        let x = &vec[position..self.position];
        return x.clone().read_i32::<BigEndian>().unwrap();
    }

    pub fn read_uint_32(&mut self) -> u32 {
        /*let position = self.position.clone();
        self.position += 4;

        let vec = &mut self.byte_array.clone();

        let x = &vec[position..self.position];
        return x.clone().read_u32::<BigEndian>().unwrap();*/
        let vec = &mut self.byte_array.clone();
        let position = self.position;
        self.position += 4;
        let bytes = &vec[position..self.position];

        println!("Bytes: {:?}", bytes);
        let mut empty_bytes: [u8; 4] = [0; 4];
        for i in 0..bytes.len() {
            empty_bytes[i] = bytes[i];
        }
        let float = unsafe { transmute::<[u8; 4], u32>(empty_bytes) };
        // let float = f32::from_be_bytes(empty_bytes);*/
        return float;
    }

    pub fn read_float(&mut self) -> f32 {
        let vec = &mut self.byte_array.clone();
        let position = self.position;
        self.position += 4;
        let bytes = &vec[position..self.position];

        println!("Bytes: {:?}", bytes);
        let mut empty_bytes: [u8; 4] = [0; 4];
        for i in 0..bytes.len() {
            empty_bytes[i] = bytes[i];
        }
        let float = unsafe { transmute::<[u8; 4], f32>(empty_bytes) };
        // let float = f32::from_be_bytes(empty_bytes);*/
        return float;
    }

    pub fn read_string(&mut self) -> String {
        let position = self.position.clone();
        self.position += self.read_packed_uint_32() as usize;

        let vec = &mut self.byte_array.clone();

        let x = &vec[position..self.position];
        return std::str::from_utf8(x).unwrap().to_string();
    }

    pub fn read_packed_uint_32(&mut self) -> i8 {
        let mut read_more = true;
        let mut output: u8 = 0;
        let mut shift = 0;
        while read_more {
            let mut byte: u8 = self.read_byte() as u8;
            if byte >= 0x80 {
                read_more = true;
                byte ^= 0x80;
            } else {
                read_more = false;
            }

            output |= byte.checked_shl(shift).unwrap_or(0) as u8;
            shift += 7;
        }
        return i8::try_from(output).expect("Unable to convert output to i8");
    }

    pub fn read_bool(&mut self) -> bool {
        let byte = self.read_byte();
        println!("Boolean: {}", byte);
        byte != 0
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.byte_array.insert(self.position, byte);
        self.position += 1;
    }

    pub fn write_uint_16(&mut self, int: u16) {
        for x in int.to_be_bytes() {
            self.write_byte(x);
        }
        self.position += 2;
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn array(&self) -> Vec<u8> {
        self.clone().byte_array
    }

    pub fn size(&self) -> usize {
        self.byte_array.len()
    }

    pub fn clone(&self) -> Buffer {
        Buffer {
            position: self.position,
            byte_array: (&self).byte_array.clone(),
        }
    }

    pub fn from(buffer: &Buffer) -> Self {
        Self {
            position: buffer.position,
            byte_array: buffer.clone().byte_array,
        }
    }
}
