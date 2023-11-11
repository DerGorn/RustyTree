use crate::Position;
use crate::Vector;

pub struct Camera {
    origin: Vector,
}
impl Camera {
    pub fn new(origin: Vector) -> Self {
        Self { origin }
    }

    pub fn project(&self, vector: &Vector) -> Position {
        Position::from_vector(vector + &self.origin)
    }
}
