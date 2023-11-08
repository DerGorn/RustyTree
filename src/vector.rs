use std::fmt::Display;

pub struct Vector {
    pub x: u32,
    pub y: u32,
}
impl Vector {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
