use crate::Buffer;

const RANGE: FloatRange = FloatRange {
    min: -50.0,
    max: 50.0
};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct FloatRange {
    pub min: f32,
    pub max: f32
}

impl FloatRange {
    pub fn lerp(&self, mut val: f32) -> f32 {
        if 0.0 > val {
            val = 0.0;
        } else if 1.0 < val {
            val = 1.0
        }
        self.min + ((self.max - self.min) * val)
    }

    pub fn reverse_lerp(&self, mut val: f32) -> f32 {
        val = (val - self.min) / (self.max - self.min);
        if 0.0 > val {
            val = 0.0
        } else if 1.0 < val {
            val = 1.0
        }
        val
    }
}

impl Vector2 {
    pub fn read_vector2(buffer: &mut Buffer) -> Self {
        let x: f32 = (buffer.read_u16_le() as f32) / 65535.0;
        let y: f32 = (buffer.read_u16_le() as f32) / 65535.0;
        Vector2 {
            x: RANGE.lerp(x),
            y: RANGE.lerp(y)
        }
    }

    pub fn write_vector2(&self, buffer: &mut Buffer) {
        let x: u16 = (RANGE.reverse_lerp(self.x) * 65535.0) as u16;
        let y: u16 = (RANGE.reverse_lerp(self.y) * 65535.0) as u16;

        buffer.write_u16_le(x);
        buffer.write_u16_le(y);
    }
}