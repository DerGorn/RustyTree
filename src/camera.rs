use crate::{math_2d::Vector, position::Position};

pub struct Camera {
    origin: Vector,
}
impl Camera {
    pub fn new(origin: Vector) -> Self {
        Self { origin }
    }

    ///Turns a `vector` in Logicalspace into one in BUfferspace, while clamping it into positive values for `Position`
    pub fn clamped_projection_to_position(&self, vector: &Vector) -> Position {
        let mut position = self.project(vector);
        if position.x < 0.0 {
            position.x = 0.0;
        }
        if position.y < 0.0 {
            position.y = 0.0;
        }
        position.into()
    }

    ///Turns a `vector` in Logicalspace into one in Bufferspace
    pub fn project(&self, vector: &Vector) -> Vector {
        let position = vector + &self.origin;
        position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamped_projection() {
        let v = Vector::new(-1.0, 15.0);
        let cam = Camera::new(Vector::new(0.0, -15.0));

        assert_eq!(cam.clamped_projection_to_position(&v), Position::new(0, 0));
    }
}
