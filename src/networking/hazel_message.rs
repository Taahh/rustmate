use crate::convert;
use crate::networking::buffer::Buffer;
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, Clone)]
pub struct HazelMessage {
    length: u16,
    tag: u8,
    payload: Buffer,
}

impl HazelMessage {
    pub fn length(&self) -> u16 {
        self.length
    }
    pub fn tag(&self) -> u8 {
        self.tag
    }
    pub fn payload(&mut self) -> &mut Buffer {
        self.payload.borrow_mut()
    }

    pub fn set_length(&mut self, length: u16) {
        self.length = length;
    }
    pub fn set_tag(&mut self, tag: u8) {
        self.tag = tag;
    }
    pub fn set_payload(&mut self, payload: Buffer) {
        self.payload = payload;
    }

    pub fn read(buffer: &mut Buffer) -> Option<Self> {
        let mut hazel_msg = Self {
            length: 0,
            tag: 0,
            payload: buffer.clone(),
        };

        if buffer.position() + 2 < buffer.size() {
            hazel_msg.set_length(buffer.read_uint_16());
        } else {
            return None;
        }
        if buffer.position() + 1 < buffer.size() {
            hazel_msg.set_tag(buffer.read_byte() as u8);
        } else {
            return None;
        }

        hazel_msg.set_payload(Buffer::new((&buffer.array()[buffer.position()..]).to_vec()));
        return Some(hazel_msg);
    }

    pub fn start_message(tag: u8) -> Self {
        let mut me = Self {
            length: 0,
            tag,
            payload: Buffer::new(Vec::new()),
        };
        me.payload.write_byte(me.tag);
        return me;
    }

    pub fn end_message(&mut self) {
        let position = self.payload.position();
        self.payload.set_position(0);
        println!("Writing hazel payload size: {}", self.payload.size());
        self.payload.write_uint_16((self.payload.size() - 1) as u16);
        self.payload.set_position(position);
    }

    /*pub fn read_all(buffer: &mut Buffer) -> Vec<HazelMessage> {
        let mut msgs = Vec::new();
        loop {
            let mut buff = &mut *buffer;
            let y = HazelMessage::read(buff);
            if y.is_some() {
                let hazel = y.clone().expect("Unable to unwrap");

                buff = &mut Buffer::from(y.clone().unwrap().payload()).clone().borrow_mut();
                msgs.push(hazel);
            } else {
                break;
            }
        }
        return msgs;
    }*/

    pub fn from(hazel_msg: &HazelMessage) -> Self {
        Self {
            length: hazel_msg.length,
            tag: hazel_msg.tag,
            payload: Buffer::from(&hazel_msg.payload),
        }
    }
}
