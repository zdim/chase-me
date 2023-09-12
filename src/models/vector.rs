pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn normalize(&mut self) {
        let magnitude = (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
        if magnitude > 1.0 {
            self.x /= magnitude;
            self.y /= magnitude;
        }
    }

    pub fn is_empty(&mut self) -> bool {
        return self.x == 0.0 || self.y == 0.0;
    }
}
