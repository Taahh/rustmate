use crate::util::buffer::Buffer;

#[derive(Debug)]
pub struct HazelMessage {
    pub length: u16,
    pub tag: i8,
    pub buffer: Buffer,
}

impl HazelMessage {
    pub fn read_message(buffer: &mut Buffer) -> Self {
        let length = buffer.read_u16();
        let tag = buffer.read_i8();
        let mut newBuffer = Buffer::new(buffer);
        HazelMessage {
            length,
            tag,
            buffer: newBuffer,
        }
    }

    pub fn start_message(tag: i8) -> Self {
        let mut buffer = Buffer {
            array: Vec::new(),
            position: 0,
        };
        buffer.write_i8(tag);
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
        buffer.array.append(&mut self.buffer.array);
        buffer.position += self.buffer.position;
    }
}
