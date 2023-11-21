use std::fmt::Display;

use crate::math_2d::Vector;

#[derive(Clone, Debug)]
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

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use crate::Res;
    use std::fmt::Write;

    use super::*;

    #[test]
    fn create() {
        let p1 = Position::new(1, 1);
        let p2 = Position::from_vector(Vector::new(1.0, 1.0));

        assert_eq!(p1, Position { x: 1, y: 1 });
        assert_eq!(p1, p2);
    }

    #[test]
    fn display() -> Res<()> {
        let mut f = String::new();
        write!(f, "{}", Position::new(1, 2))?;
        assert_eq!(f, "(x: 1, y: 2)");
        Ok(())
    }
}
