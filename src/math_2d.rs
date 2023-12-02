mod matrix;
use std::ops::Mul;

pub use matrix::Matrix;

mod vector;
pub use vector::{Vector, Intersection};

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

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn rotate_vector() {
        let origin = Vector::new(1.0, 1.0);
        let x = Vector::new(1.0, 0.0);
        let y = x.rotate_degree(90.0);

        assert_eq!(y, Vector::new(6.123233995736766e-17, 1.0));
        assert_eq!(y, x.rotate(PI / 2.0));
        assert_eq!(
            x.rotate_around(PI / 2.0, &origin),
            Vector {
                x: 2.0,
                y: 0.9999999999999999
            }
        );
    }

    #[test]
    fn mul_vector_matrix() {
        let m = Matrix::scalar(1.0);
        let v = Vector::scalar(2.0);

        assert_eq!(&m * &v, Vector::scalar(4.0));
        assert_eq!(&m * &v, m.clone() * v.clone());
        assert_eq!(&m * &v, m.clone() * &v);
        assert_eq!(&m * &v, &m * v.clone());
    }
}
