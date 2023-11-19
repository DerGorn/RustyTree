use std::f64::consts::PI;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// (a b
///  c d)
#[derive(Clone, Debug)]
pub struct Matrix {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}
impl Matrix {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self { a, b, c, d }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn unity() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    pub fn rotation(radians: f64) -> Self {
        Self::new(radians.cos(), -radians.sin(), radians.sin(), radians.cos())
    }

    pub fn rotation_degree(degree: f64) -> Self {
        let radians = degree * PI / 180.0;
        Self::rotation(radians)
    }

    pub fn scalar(value: f64) -> Self {
        Self {
            a: value,
            b: value,
            c: value,
            d: value,
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d
    }

    fn ne(&self, other: &Self) -> bool {
        self.a != other.a || self.b != other.b || self.c != other.c || self.d != other.d
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(({}, {}), ({}, {}))", self.a, self.b, self.c, self.d)
    }
}

//Add
impl Add for Matrix {
    type Output = Matrix;

    fn add(self, _rhs: Self) -> Matrix {
        Matrix::new(
            self.a + _rhs.a,
            self.b + _rhs.b,
            self.c + _rhs.c,
            self.d + _rhs.d,
        )
    }
}
impl Add<&Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, _rhs: &Self) -> Matrix {
        Matrix::new(
            self.a + _rhs.a,
            self.b + _rhs.b,
            self.c + _rhs.c,
            self.d + _rhs.d,
        )
    }
}
impl Add for &Matrix {
    type Output = Matrix;

    fn add(self, _rhs: Self) -> Matrix {
        Matrix::new(
            self.a + _rhs.a,
            self.b + _rhs.b,
            self.c + _rhs.c,
            self.d + _rhs.d,
        )
    }
}
impl Add<Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, _rhs: Matrix) -> Matrix {
        Matrix::new(
            self.a + _rhs.a,
            self.b + _rhs.b,
            self.c + _rhs.c,
            self.d + _rhs.d,
        )
    }
}
//AddAssign
impl AddAssign for Matrix {
    fn add_assign(&mut self, _rhs: Self) {
        self.a += _rhs.a;
        self.b += _rhs.b;
        self.c += _rhs.c;
        self.d += _rhs.d;
    }
}
impl AddAssign<&Matrix> for Matrix {
    fn add_assign(&mut self, _rhs: &Self) {
        self.a += _rhs.a;
        self.b += _rhs.b;
        self.c += _rhs.c;
        self.d += _rhs.d;
    }
}

//Mul
impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a * rhs.a + self.b * rhs.c,
            self.a * rhs.b + self.b * rhs.d,
            self.c * rhs.a + self.d * rhs.c,
            self.c * rhs.b + self.d * rhs.d,
        )
    }
}
impl Mul<&Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Self) -> Self::Output {
        Self::new(
            self.a * rhs.a + self.b * rhs.c,
            self.a * rhs.b + self.b * rhs.d,
            self.c * rhs.a + self.d * rhs.c,
            self.c * rhs.b + self.d * rhs.d,
        )
    }
}
impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix::new(
            self.a * rhs.a + self.b * rhs.c,
            self.a * rhs.b + self.b * rhs.d,
            self.c * rhs.a + self.d * rhs.c,
            self.c * rhs.b + self.d * rhs.d,
        )
    }
}
impl Mul<Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix::new(
            self.a * rhs.a + self.b * rhs.c,
            self.a * rhs.b + self.b * rhs.d,
            self.c * rhs.a + self.d * rhs.c,
            self.c * rhs.b + self.d * rhs.d,
        )
    }
}
impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        Matrix::new(rhs * self.a, rhs * self.b, rhs * self.c, rhs * self.d)
    }
}
impl Mul<&f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &f64) -> Self::Output {
        Matrix::new(rhs * self.a, rhs * self.b, rhs * self.c, rhs * self.d)
    }
}
impl Mul<f64> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        Matrix::new(rhs * self.a, rhs * self.b, rhs * self.c, rhs * self.d)
    }
}
impl Mul<&f64> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &f64) -> Self::Output {
        Matrix::new(rhs * self.a, rhs * self.b, rhs * self.c, rhs * self.d)
    }
}
impl Mul<Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix::new(self * rhs.a, self * rhs.b, self * rhs.c, self * rhs.d)
    }
}
impl Mul<&Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        Matrix::new(self * rhs.a, self * rhs.b, self * rhs.c, self * rhs.d)
    }
}
impl Mul<Matrix> for &f64 {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix::new(self * rhs.a, self * rhs.b, self * rhs.c, self * rhs.d)
    }
}
impl Mul<&Matrix> for &f64 {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        Matrix::new(self * rhs.a, self * rhs.b, self * rhs.c, self * rhs.d)
    }
}
//MulAssign
impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Self) {
        let a = self.a * rhs.a + self.b * rhs.c;
        let b = self.a * rhs.b + self.b * rhs.d;
        let c = self.c * rhs.a + self.d * rhs.c;
        let d = self.c * rhs.b + self.d * rhs.d;
        self.a = a;
        self.b = b;
        self.c = c;
        self.d = d;
    }
}
impl MulAssign<&Matrix> for Matrix {
    fn mul_assign(&mut self, rhs: &Self) {
        let a = self.a * rhs.a + self.b * rhs.c;
        let b = self.a * rhs.b + self.b * rhs.d;
        let c = self.c * rhs.a + self.d * rhs.c;
        let d = self.c * rhs.b + self.d * rhs.d;
        self.a = a;
        self.b = b;
        self.c = c;
        self.d = d;
    }
}
impl MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, rhs: f64) {
        self.a *= rhs;
        self.b *= rhs;
        self.c *= rhs;
        self.d *= rhs;
    }
}
impl MulAssign<&f64> for Matrix {
    fn mul_assign(&mut self, rhs: &f64) {
        self.a *= rhs;
        self.b *= rhs;
        self.c *= rhs;
        self.d *= rhs;
    }
}

//Neg
impl Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}
impl Neg for &Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

//Sub
impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Self::Output {
        Matrix::new(
            self.a - rhs.a,
            self.b - rhs.b,
            self.c - rhs.c,
            self.d - rhs.d,
        )
    }
}
impl Sub<&Matrix> for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: &Self) -> Self::Output {
        Matrix::new(
            self.a - rhs.a,
            self.b - rhs.b,
            self.c - rhs.c,
            self.d - rhs.d,
        )
    }
}
impl Sub for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Self::Output {
        Matrix::new(
            self.a - rhs.a,
            self.b - rhs.b,
            self.c - rhs.c,
            self.d - rhs.d,
        )
    }
}
impl Sub<Matrix> for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Matrix) -> Self::Output {
        Matrix::new(
            self.a - rhs.a,
            self.b - rhs.b,
            self.c - rhs.c,
            self.d - rhs.d,
        )
    }
}
//SubAssign
impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        self.a -= rhs.a;
        self.b -= rhs.b;
        self.c -= rhs.c;
        self.d -= rhs.d;
    }
}
impl SubAssign<&Matrix> for Matrix {
    fn sub_assign(&mut self, rhs: &Self) {
        self.a -= rhs.a;
        self.b -= rhs.b;
        self.c -= rhs.c;
        self.d -= rhs.d;
    }
}

#[cfg(test)]
mod tests {
    use crate::Res;
    use std::f64::consts::PI;
    use std::fmt::Write;

    use super::*;

    #[test]
    fn creation() {
        let reference = Matrix {
            a: 1.0,
            b: 1.0,
            c: 1.0,
            d: 1.0,
        };

        assert_eq!(Matrix::new(1.0, 1.0, 1.0, 1.0), reference);
        assert_eq!(Matrix::scalar(1.0), reference);
        assert_eq!(Matrix::zero(), Matrix::scalar(0.0));
        assert_eq!(
            Matrix::unity(),
            Matrix {
                a: 1.0,
                b: 0.0,
                c: 0.0,
                d: 1.0,
            }
        )
    }

    #[test]
    fn rotation() {
        let alpha = 1.0_f64;
        let reference = Matrix {
            a: alpha.cos(),
            b: -alpha.sin(),
            c: alpha.sin(),
            d: alpha.cos(),
        };

        assert_eq!(Matrix::rotation(alpha), reference);
        assert_eq!(Matrix::rotation_degree(alpha * 180.0 / PI), reference);
    }

    #[test]
    fn display() -> Res<()> {
        let mut f = String::new();
        write!(f, "{}", Matrix::unity())?;
        assert_eq!(f, "((1, 0), (0, 1))");
        Ok(())
    }

    #[test]
    fn add() {
        let x = Matrix::unity();
        let y = Matrix::scalar(1.0);
        let reference = &x + &y;

        assert_eq!(reference, Matrix::new(2.0, 1.0, 1.0, 2.0));
        assert_eq!(reference, x.clone() + y.clone());
        assert_eq!(reference, y.clone() + x.clone());
        assert_eq!(reference, x.clone() + &y);
        assert_eq!(reference, &y + x.clone());
    }

    #[test]
    fn add_asign() {
        let mut x1 = Matrix::unity();
        let mut x2 = Matrix::unity();
        let y = Matrix::scalar(1.0);

        x2 += &y;
        x1 += y;
        assert_eq!(x1, Matrix::new(2.0, 1.0, 1.0, 2.0));
        assert_eq!(x1, x2)
    }

    #[test]
    fn mul() {
        let x = Matrix::unity();
        let y = Matrix::scalar(1.0);
        let reference = &x * &y;

        assert_eq!(reference, Matrix::scalar(1.0));
        assert_eq!(reference, x.clone() * y.clone());
        assert_eq!(reference, y.clone() * x.clone());
        assert_eq!(reference, x.clone() * &y);
        assert_eq!(reference, &y * x.clone());
    }

    #[test]
    fn mul_f64() {
        let x = Matrix::unity();
        let y = 2.0;
        let reference = &x * &y;

        assert_eq!(reference, Matrix::new(2.0, 0.0, 0.0, 2.0));
        assert_eq!(reference, x.clone() * y);
        assert_eq!(reference, x.clone() * &y);
        assert_eq!(reference, &x * y);
        assert_eq!(reference, y * x.clone());
        assert_eq!(reference, y * &x);
        assert_eq!(reference, &y * x.clone());
        assert_eq!(reference, &y * &x);
    }

    #[test]
    fn mul_assign() {
        let mut x1 = Matrix::unity();
        let mut x2 = Matrix::unity();
        let y = Matrix::scalar(1.0);

        x1 *= &y;
        x2 *= y.clone();
        assert_eq!(x1, Matrix::scalar(1.0));
        assert_eq!(x1, x2)
    }

    #[test]
    fn mul_f64_assign() {
        let mut x_1 = Matrix::unity();
        let mut x_2 = Matrix::unity();
        let y = 2.0;

        x_1 *= &y;
        x_2 *= y;
        assert_eq!(x_1, Matrix::new(2.0, 0.0, 0.0, 2.0));
        assert_eq!(x_1, x_2)
    }

    #[test]
    fn neg() {
        let x = Matrix::scalar(1.0);
        assert_eq!(-&x, Matrix::scalar(-1.0));
        assert_eq!(-x, Matrix::scalar(-1.0));
    }

    #[test]
    fn sub() {
        let x = Matrix::unity();
        let y = Matrix::scalar(1.0);
        let reference = &x - &y;

        assert_eq!(reference, Matrix::new(0.0, -1.0, -1.0, 0.0));
        assert_eq!(reference, x.clone() - y.clone());
        assert_eq!(reference, x.clone() - &y);
        assert_eq!(reference, &x - y.clone());
        assert_eq!(reference, -(y.clone() - x.clone()));
    }

    #[test]
    fn sub_asign() {
        let mut x1 = Matrix::unity();
        let mut x2 = Matrix::unity();
        let y = Matrix::scalar(1.0);

        x2 -= &y;
        x1 -= y;
        assert_eq!(x1, Matrix::new(0.0, -1.0, -1.0, 0.0));
        assert_eq!(x1, x2)
    }
}
