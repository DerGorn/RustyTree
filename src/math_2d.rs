mod matrix;
use std::ops::Mul;

pub use matrix::Matrix;

mod vector;
pub use vector::Vector;

impl Vector {
    pub fn rotate(&self, radians: f64) -> Vector {
        Matrix::rotation(radians) * self
    }
    pub fn rotate_degree(&self, degree: f64) -> Vector {
        Matrix::rotation_degree(degree) * self
    }

    pub fn rotate_around(&self, radians: f64, origin: &Vector) -> Vector {
        let vector = self - origin;
        vector.rotate(radians) + origin
    }
    pub fn rotate_degree_around(&self, degree: f64, origin: &Vector) -> Vector {
        let vector = self - origin;
        vector.rotate_degree(degree) + origin
    }

    pub fn distance(&self, other: &Vector) -> f64 {
        (self - other).length().sqrt()
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self.a * rhs.x + self.b * rhs.y,
            self.c * rhs.x + self.d * rhs.y,
        )
    }
}
impl Mul<&Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        Vector::new(
            self.a * rhs.x + self.b * rhs.y,
            self.c * rhs.x + self.d * rhs.y,
        )
    }
}
impl Mul<Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self.a * rhs.x + self.b * rhs.y,
            self.c * rhs.x + self.d * rhs.y,
        )
    }
}
impl Mul<&Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        Vector::new(
            self.a * rhs.x + self.b * rhs.y,
            self.c * rhs.x + self.d * rhs.y,
        )
    }
}
