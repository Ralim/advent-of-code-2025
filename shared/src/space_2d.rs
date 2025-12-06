#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYPos {
    pub x: f64,
    pub y: f64,
}

impl XYPos {
    pub fn new(x: f64, y: f64) -> Self {
        XYPos { x, y }
    }
    pub fn distance_to(&self, other: &XYPos) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    pub fn move_by(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}
