use tracing::info;
use crate::util::buffer::Buffer;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct HazelMessage {
    pub length: u16,
    pub tag: u8,
    pub buffer: Buffer,
}

impl HazelMessage {
    pub fn read_message(buffer: &mut Buffer) -> Option<Self> {
        if buffer.position >= buffer.array.len() {
            return None;
        }
        let length = buffer.read_u16_le();
        if buffer.position >= buffer.array.len() {
            return None;
        }
        let tag = buffer.read_u8();
        if buffer.position >= buffer.array.len() {
            return None;
        }
        let mut newBuffer = buffer.clone();
        buffer.position += length as usize;
        Some(HazelMessage {
            length,
            tag,
            buffer: newBuffer,
        })
    }

    pub fn start_message(tag: u8) -> Self {
        let mut buffer = Buffer {
            array: Vec::new(),
            position: 0,
        };
        buffer.write_u8(tag);
        HazelMessage {
            length: 0,
            tag,
            buffer,
        }
    }

    pub fn end_message(&mut self) {
        let length = self.buffer.array.len();
        let position = self.buffer.position;
        self.buffer.position = 0;
        self.buffer.write_u16_le(length as u16 - 1);
        self.buffer.position = position;
    }

    pub fn copy_to(&mut self, buffer: &mut Buffer) {
        buffer.write_u8_arr_le(&*self.buffer.array);
        // buffer.array.append(&mut self.buffer.array);
        // buffer.position += self.buffer.position;
    }
}
