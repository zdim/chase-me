use sdl2::rect::Rect;

use crate::models::position::Position;

pub struct Camera {
    pub size: Rect,
    pub location: Position,
}

