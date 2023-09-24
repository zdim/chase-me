use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Renderer {
    pub screen_area: Rect,
    pub clear_color: Color,
}

impl Renderer {
    pub fn clear(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.clear_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default();
    }
}
