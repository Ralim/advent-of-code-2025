#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYZPos {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl XYZPos {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        XYZPos { x, y, z }
    }
    pub fn distance_to(&self, other: &XYZPos) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
    pub fn move_by(&mut self, dx: f64, dy: f64, dz: f64) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }
}
