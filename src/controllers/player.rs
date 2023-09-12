use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::models::position::Position;
use crate::models::vector::Vector;

const WIDTH: u32 = 20;
const HEIGHT: u32 = 20;

pub struct Player {
    pub speed: f32,
    pub location: Position,
    pub destination: Option<Position>,
    pub direction: Vector,
}

impl Player {
    fn r#move(&mut self, delta: f64) {
        if self.destination.is_none() {
            return;
        }

        let destination = self.destination.unwrap();
        if self.location == destination {
            self.destination = None;
            return;
        }

        let mut direction = Vector {
            x: (destination.x - self.location.x),
            y: (destination.y - self.location.y),
        };

        direction.normalize();

        self.location.x += (self.speed * delta as f32) * direction.x;
        self.location.y += (self.speed * delta as f32) * direction.y;

        // clamp to our destination
        if direction.x < 0.0 {
            if self.location.x < destination.x {
                self.location.x = destination.x
            }
        } else {
            if self.location.x > destination.x {
                self.location.x = destination.x
            }
        }

        if direction.y < 0.0 {
            if self.location.y < destination.y {
                self.location.y = destination.y
            }
        } else {
            if self.location.y > destination.y {
                self.location.y = destination.y
            }
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.r#move(delta)
    }

    pub fn set_destination(&mut self, destination: Position) {
        self.destination = Some(destination);
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGBA(255, 255, 255, 1));
        canvas
            .fill_rect(Rect::new(
                (self.location.x as i32) - (WIDTH as i32 / 2),
                self.location.y as i32 - (HEIGHT as i32 / 2),
                WIDTH,
                HEIGHT,
            ))
            .ok()
            .unwrap_or_default();

        // render the destination. should this be in a different file?
        // this will probably be a texture at some point
        canvas.set_draw_color(Color::RGBA(255, 50, 50, 1));
        if self.destination.is_some() {
            canvas
                .draw_line(
                    Point::new(
                        self.destination.unwrap().x as i32 - 5,
                        self.destination.unwrap().y as i32 - 5,
                    ),
                    Point::new(
                        self.destination.unwrap().x as i32 + 5,
                        self.destination.unwrap().y as i32 + 5,
                    ),
                )
                .ok()
                .unwrap_or_default();
        
            canvas.draw_line(
                Point::new(
                    self.destination.unwrap().x as i32 + 5,
                    self.destination.unwrap().y as i32 - 5, 
                ),
                Point::new( 
                    self.destination.unwrap().x as i32 - 5,
                    self.destination.unwrap().y as i32 + 5,
                ),
            ).ok().unwrap_or_default();
        }
    }
}
