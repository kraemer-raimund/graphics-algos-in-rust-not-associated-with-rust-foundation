#[derive(Debug, Clone)]
pub struct Circle {
    x: u16,
    y: u16,
    radius: u16,
}

impl Circle {
    pub fn new(x: u16, y: u16, radius: u16) -> Self {
        Self { x, y, radius }
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn radius(&self) -> u16 {
        self.radius
    }
}
