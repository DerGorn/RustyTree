use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::debug::debug_print;

/// Describes the Intersection between two vectors `v1` and `v2`.
/// Where the relative Position from `v2` to `v1` is given as `s`
#[derive(Debug, Clone, PartialEq)]
pub enum Intersection {
    ///No Intersection
    None,
    ///One Intersection `Point(r)` at `v1 * r`
    Point(f64),
    ///Overlap `Line(r, t)` in a section from `v1 * r` to `v1 * t`
    Line(f64, f64),
}

#[derive(Debug, Clone, PartialEq)]
enum SlopeQuadrant {
    TopRight(f64),
    TopLeft(f64),
    BottomLeft(f64),
    BottomRight(f64),
}
impl SlopeQuadrant {
    fn opposite(&self) -> SlopeQuadrant {
        match self {
            Self::BottomLeft(slope) => Self::TopRight(*slope),
            Self::BottomRight(slope) => Self::TopLeft(*slope),
            Self::TopLeft(slope) => Self::BottomRight(*slope),
            Self::TopRight(slope) => Self::BottomLeft(*slope),
        }
    }

    fn is_opposite(&self, other: &SlopeQuadrant) -> bool {
        self.opposite() == *other
    }

    fn slope(&self) -> f64 {
        match self {
            Self::BottomLeft(slope) => *slope,
            Self::BottomRight(slope) => *slope,
            Self::TopLeft(slope) => *slope,
            Self::TopRight(slope) => *slope,
        }
    }
}
impl Mul for &SlopeQuadrant {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        (if self.is_opposite(&rhs) { -1.0 } else { 1.0 }) * (self.slope() * rhs.slope()).abs()
    }
}

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

    fn overlap_area(&self, other: &Vector, support: &Vector) -> Intersection {
        let self_len = self.length();
        let support_len = support.length();
        let other_len = other.length();
        let support_fract = support_len / self_len;

        let self_slope = self.slope();
        let support_slope = support.slope();
        let other_slope = other.slope();
        let support_direction = &self_slope * &support_slope > 0.0;
        let line_direction = &self_slope * &other_slope > 0.0;

        debug_print!("self_len" self_len, "support_len" support_len, "other_len" other_len, "support_fract" support_fract, "self_slope" self_slope, "suppoert_slope" support_slope, "other_slope" other_slope, "suppoert_direction" support_direction, "line_direction" line_direction);
        if line_direction && support_direction {
            //v1.slope = v2.slope && v1.slope = support.slope
            if self_len < support_len {
                Intersection::None
            } else if self_len == support_len {
                Intersection::Point(1.0)
            } else if self_len < other_len {
                Intersection::Line(support_fract, 1.0)
            } else {
                Intersection::Line(support_fract, other_len / self_len)
            }
        } else if !line_direction && !support_direction {
            //v1.slope = - v2.slope && v1.slope = - support.slope
            Intersection::None
        } else if line_direction && !support_direction {
            //v1.slope =  v2.slope && v1.slope = - support.slope
            if support_len > other_len {
                Intersection::None
            } else if support_len == other_len {
                Intersection::Point(0.0)
            } else if support_len + self_len > other_len {
                Intersection::Line(0.0, (other_len - support_len) / self_len)
            } else {
                Intersection::Line(0.0, 1.0)
            }
        } else {
            //v1.slope = support.slope && v1.slope = - v2.slope
            if self_len < support_len {
                let diff = support_len - self_len;
                if other_len < diff {
                    Intersection::None
                } else if other_len == diff {
                    Intersection::Point(1.0)
                } else {
                    let diff = other_len - diff;
                    if diff >= self_len {
                        Intersection::Line(0.0, 1.0)
                    } else {
                        Intersection::Line(1.0 - diff / self_len, 1.0)
                    }
                }
            } else {
                if support_len == 0.0 {
                    Intersection::Point(0.0)
                } else if other_len >= support_len {
                    Intersection::Line(0.0, support_fract)
                } else {
                    Intersection::Line(support_fract - other_len / self_len, support_fract)
                }
            }
        }
    }

    /// Calculates the intersection point of `self` and `other`, where the relative position between
    /// them is given as `support`.
    ///
    /// If a intersection point exists the returned `Some(r)`
    /// gives the factor of self at which the intersection point lies.
    /// If the
    pub fn intersection(&self, other: &Vector, support: &Vector) -> Intersection {
        let mut swapped_vecs = false;
        debug_print!("self" self, "other" other, "suppoert" support);
        let (v1, v2) = if other.y == 0.0 {
            swapped_vecs = true;
            (other, self)
        } else {
            (self, other)
        };
        debug_print!("v1" v1, "v2" v2);
        let s_y = support.y - support.x * v1.y / v1.x;
        debug_print!("s_y" s_y);
        let t = if v1.y == 0.0 {
            -s_y / v2.y
        } else {
            let divisor = -v2.y + v1.y * v2.x / v1.x;
            debug_print!("divisor" divisor);
            if divisor == 0.0 {
                f64::INFINITY
            } else {
                s_y / divisor
            }
        };
        debug_print!("t" t);
        if t.is_infinite() {
            // t is variable || 0 * t = s =/= 0
            return if s_y != 0.0 {
                // 0 * t = s
                Intersection::None
            } else {
                // t is variable => Line Overlap
                self.overlap_area(other, support)
            };
        }
        if t < 0.0 || t > 1.0 {
            return Intersection::None;
        }
        let r = if v1.x == 0.0 {
            f64::INFINITY
        } else {
            (support.x + v2.x * t) / v1.x
        };
        debug_print!("r" r);
        if r.is_infinite() {
            // r is variable || 0 * r = s =/= 0
            return if support.x != 0.0 {
                // 0 * r = s
                Intersection::None
            } else {
                // r is variable => Line Overlap
                self.overlap_area(other, support)
            };
        }
        if r < 0.0 || r > 1.0 {
            return Intersection::None;
        }
        let intersection = r * v1;
        if (&intersection - (support + t * v2)).length() > 1e-8 {
            return Intersection::None;
        }
        Intersection::Point(if swapped_vecs { t } else { r })
    }

    fn slope(&self) -> SlopeQuadrant {
        let mut slope = (self.y / self.x).abs();
        if slope.is_nan() {
            slope = f64::INFINITY;
        }
        if self.x > 0.0 {
            if self.y >= 0.0 {
                SlopeQuadrant::TopRight(slope)
            } else {
                SlopeQuadrant::BottomRight(slope)
            }
        } else {
            if self.y >= 0.0 {
                SlopeQuadrant::TopLeft(slope)
            } else {
                SlopeQuadrant::BottomLeft(slope)
            }
        }
    }

    pub fn round(&self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }

    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::zero()
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

//Div
impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs)
    }
}
impl Div<&f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: &f64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs)
    }
}
impl Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs)
    }
}
impl Div<&f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: &f64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs)
    }
}
//DivAssign
impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}
impl DivAssign<&f64> for Vector {
    fn div_assign(&mut self, rhs: &f64) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
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

    #[test]
    fn intersection() {
        let v1 = Vector::new(10.0, 10.0);
        let v2 = Vector::new(-10.0, -10.0);
        let v3 = Vector::new(10.0, -10.0);

        assert_eq!(
            v1.intersection(&v2, &Vector::zero()),
            Intersection::Point(0.0)
        );
        assert_eq!(
            v1.intersection(&v2, &Vector::scalar(-1.0)),
            Intersection::None
        );
        assert_eq!(
            v1.intersection(&v2, &Vector::scalar(1.0)),
            Intersection::Line(0.0, 0.1)
        );
        assert_eq!(
            v1.intersection(&v2, &Vector::scalar(10.0)),
            Intersection::Line(0.0, 1.0)
        );
        assert_eq!(
            v1.intersection(&v2, &Vector::scalar(11.0)),
            Intersection::Line(0.09999999999999998, 1.0)
        );
        assert_eq!(
            v1.intersection(&-&v2, &Vector::zero()),
            Intersection::Line(0.0, 1.0)
        );
        assert_eq!(
            v1.intersection(&-&v2, &Vector::scalar(1.0)),
            Intersection::Line(0.1, 1.0)
        );
        assert_eq!(
            v1.intersection(&-&v2, &Vector::scalar(-1.0)),
            Intersection::Line(0.0, 0.8999999999999999)
        );
        assert_eq!(
            v1.intersection(&-&v2, &Vector::scalar(10.0)),
            Intersection::Point(1.0)
        );
        assert_eq!(
            v1.intersection(&-&v2, &Vector::scalar(11.0)),
            Intersection::None
        );
        assert_eq!(
            v1.intersection(&v3, &Vector::new(0.0, 10.0)),
            Intersection::Point(0.5)
        );
        assert_eq!(
            v1.intersection(&-&v3, &Vector::new(0.0, 10.0)),
            Intersection::None
        );
        assert_eq!(
            v1.intersection(&-&v3, &Vector::new(10.0, 0.0)),
            Intersection::Point(0.5)
        );
    }
}
