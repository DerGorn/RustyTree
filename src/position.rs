use std::fmt::Display;

use crate::math_2d::Vector;

pub struct Position {
    pub x: u32,
    pub y: u32,
}
impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn from_vector(vector: Vector) -> Self {
        let x = vector.x.round();
        let y = vector.y.round();
        if x < u32::MIN as f64 {
            panic!(
                "Invalid Position. Positions only supports u32 values but 'x = {} < {}'",
                x,
                u32::MIN
            );
        };
        if x > u32::MAX as f64 {
            panic!(
                "Invalid Position. Positions only supports u32 values but 'x = {} > {}'",
                x,
                u32::MAX
            );
        };
        if y < u32::MIN as f64 {
            panic!(
                "Invalid Position. Positions only supports u32 values but 'y = {} < {}'",
                y,
                u32::MIN
            );
        };
        if y > u32::MAX as f64 {
            panic!(
                "Invalid Position. Positions only supports u32 values but 'y = {} > {}'",
                y,
                u32::MAX
            );
        };
        Position::new(x as u32, y as u32)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}
