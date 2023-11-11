use crate::Vector;

pub struct Position {
    pub x: u32,
    pub y: u32,
}
impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn from_vector(vector: Vector) -> Self {
        if vector.x < u32::MIN as f64 {
            panic!(
                "Invalid Position. Positions only supports u32 values but 'x = {} < {}'",
                vector.x,
                u32::MIN
            );
        };
        if vector.x > u32::MAX as f64 {
            panic!(
                "Invalid Position. Positions only supports u32 values but 'x = {} > {}'",
                vector.x,
                u32::MAX
            );
        };
        if vector.x < u32::MIN as f64 {
            panic!(
                "Invalid Position. Positions only supports u32 values but 'x = {} < {}'",
                vector.y,
                u32::MIN
            );
        };
        if vector.y > u32::MAX as f64 {
            panic!(
                "Invalid Position. Positions only supports u32 values but 'x = {} > {}'",
                vector.y,
                u32::MAX
            );
        };
        Position::new(vector.x as u32, vector.y as u32)
    }
}
