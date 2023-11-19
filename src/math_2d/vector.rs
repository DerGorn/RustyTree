use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}
impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn scalar(value: f64) -> Self {
        Self { x: value, y: value }
    }

    pub fn length(&self) -> f64 {
        (self * self).sqrt()
    }

    pub fn distance(&self, other: &Vector) -> f64 {
        (self - other).length()
    }

    pub fn round(&self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }

    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//Add
impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add<&Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: &Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add<Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}
//AddAssign
impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

//Mul
impl Mul for Vector {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl Mul<&Vector> for Vector {
    type Output = f64;

    fn mul(self, rhs: &Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl Mul for &Vector {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl Mul<Vector> for &Vector {
    type Output = f64;

    fn mul(self, rhs: Vector) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}
impl Mul<&f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: &f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}
impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}
impl Mul<&f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: &f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}
impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y)
    }
}
impl Mul<&Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y)
    }
}
impl Mul<Vector> for &f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y)
    }
}
impl Mul<&Vector> for &f64 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y)
    }
}
//MulAssign
impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}
impl MulAssign<&f64> for Vector {
    fn mul_assign(&mut self, rhs: &f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

//Neg
impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y)
    }
}
impl Neg for &Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y)
    }
}

//Sub
impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Sub<&Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: &Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Sub<Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}
//SubAssign
impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}
impl SubAssign<&Vector> for Vector {
    fn sub_assign(&mut self, rhs: &Vector) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use crate::Res;

    use super::*;

    #[test]
    fn new() {
        assert_eq!(Vector::new(1.0, 2.0), Vector { x: 1.0, y: 2.0 })
    }

    #[test]
    pub fn zero() {
        assert_eq!(Vector::zero(), Vector { x: 0.0, y: 0.0 })
    }

    #[test]
    pub fn scalar() {
        assert_eq!(Vector::scalar(1.0), Vector { x: 1.0, y: 1.0 })
    }

    #[test]
    pub fn length() {
        assert_eq!(Vector::scalar(2.0).length(), 8.0_f64.sqrt())
    }

    #[test]
    pub fn distance() {
        let x = Vector::scalar(2.0);
        let y = Vector::scalar(1.0);

        assert_eq!(x.distance(&y), Vector::scalar(1.0).length());
        assert_eq!(x.distance(&y), y.distance(&x));
    }

    #[test]
    pub fn round() {
        assert_eq!(
            Vector::new(1.1, 2.6).round(),
            Vector::new(1.1_f64.round(), 2.6_f64.round())
        )
    }

    #[test]
    pub fn abs() {
        assert_eq!(Vector::new(-1.0, 1.0).abs(), Vector::new(1.0, 1.0));
        assert_eq!(Vector::new(1.0, -1.0).abs(), Vector::new(1.0, 1.0));
    }

    #[test]
    fn eq() {
        assert!(Vector::scalar(1.0) == Vector::scalar(1.0))
    }

    #[test]
    fn ne() {
        assert!(Vector::scalar(2.0) != Vector::scalar(1.0))
    }

    #[test]
    fn display() -> Res<()> {
        let mut f = String::new();
        write!(f, "{}", Vector::new(1.0, 2.2))?;
        assert_eq!(f, "(1, 2.2)");
        Ok(())
    }

    #[test]
    fn add() {
        let x = Vector::new(1.0, 0.0);
        let y = Vector::new(0.0, 1.0);

        assert_eq!(&x + &y, Vector::new(1.0, 1.0));
        assert_eq!(&x + &y, &x + y.clone());
        assert_eq!(x.clone() + y.clone(), &x + y.clone());
        assert_eq!(&x + &y, x.clone() + &y);
        assert_eq!(&x + &y, &y + &x);
    }

    #[test]
    fn add_asign() {
        let mut x_1 = Vector::new(1.0, 0.0);
        let mut x_2 = Vector::new(1.0, 0.0);
        let y = Vector::new(0.0, 1.0);

        x_1 += &y;
        x_2 += y;
        assert_eq!(x_1, Vector::new(1.0, 1.0));
        assert_eq!(x_1, x_2)
    }

    #[test]
    fn mul() {
        let x = Vector::new(1.0, 0.0);
        let y = Vector::new(0.0, 1.0);

        assert_eq!(&x * &y, 0.0);
        assert_eq!(&x * &y, &x * y.clone());
        assert_eq!(&x * &y, x.clone() * &y);
        assert_eq!(x.clone() * y.clone(), &x * y.clone());
        assert_eq!(&x * &y, &y * &x);
    }

    #[test]
    fn mul_f64() {
        let x = Vector::new(1.0, 0.0);
        let y = 2.0;

        assert_eq!(&x * &y, Vector::new(2.0, 0.0));
        assert_eq!(&x * &y, &y * &x);
        assert_eq!(&x * &y, &x * y.clone());
        assert_eq!(&x * y.clone(), y.clone() * &x);
        assert_eq!(&x * &y, x.clone() * &y);
        assert_eq!(&x * &y, &y * x.clone());
        assert_eq!(x.clone() * y, y * x.clone());
        assert_eq!(x.clone() * y.clone(), &x * y.clone());
        assert_eq!(x.clone() * y, y * x.clone());
        assert_eq!(&x * &y, y * &x);
    }

    #[test]
    fn mul_f64_assign() {
        let mut x_1 = Vector::new(1.0, 0.0);
        let mut x_2 = Vector::new(1.0, 0.0);
        let y = 2.0;

        x_1 *= &y;
        x_2 *= y;
        assert_eq!(x_1, Vector::new(2.0, 0.0));
        assert_eq!(x_1, x_2)
    }

    #[test]
    fn neg() {
        let x = Vector::scalar(1.0);
        assert_eq!(-&x, Vector::scalar(-1.0));
        assert_eq!(-x, Vector::scalar(-1.0));
    }

    #[test]
    fn sub() {
        let x = Vector::new(1.0, 0.0);
        let y = Vector::new(0.0, 1.0);

        assert_eq!(&x - &y, Vector::new(1.0, -1.0));
        assert_eq!(&x - &y, &x - y.clone());
        assert_eq!(x.clone() - y.clone(), &x - y.clone());
        assert_eq!(&x - &y, x.clone() - &y);
        assert_eq!(&x - &y, -(&y - &x));
    }

    #[test]
    fn sub_asign() {
        let mut x_1 = Vector::new(1.0, 0.0);
        let mut x_2 = Vector::new(1.0, 0.0);
        let y = Vector::new(0.0, 1.0);

        x_1 -= &y;
        x_2 -= y;
        assert_eq!(x_1, Vector::new(1.0, -1.0));
        assert_eq!(x_1, x_2)
    }
}
