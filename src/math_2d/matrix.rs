use std::f64::consts::PI;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// (a b
///  c d)
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
        self.a = self.a * rhs.a + self.b * rhs.c;
        self.b = self.a * rhs.b + self.b * rhs.d;
        self.c = self.c * rhs.a + self.d * rhs.c;
        self.d = self.c * rhs.b + self.d * rhs.d;
    }
}
impl MulAssign<&Matrix> for Matrix {
    fn mul_assign(&mut self, rhs: &Self) {
        self.a = self.a * rhs.a + self.b * rhs.c;
        self.b = self.a * rhs.b + self.b * rhs.d;
        self.c = self.c * rhs.a + self.d * rhs.c;
        self.d = self.c * rhs.b + self.d * rhs.d;
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
