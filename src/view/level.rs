use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Renderer {
    pub area: Rect
}

impl Renderer {
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(50, 200, 50));
        canvas.fill_rect(self.area).ok().unwrap_or_default();
    }
}
