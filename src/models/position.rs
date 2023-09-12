use std::cmp::Eq;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Position {}
