mod body;
pub use body::Body;
pub use body::VisualShape;
pub use body::BodyBuilder;

mod ref_body;
pub use ref_body::RefBody;

mod collision;
pub use collision::CollisionLayer;
pub use collision::{CollisionBody, Mass};

use crate::math_2d::Vector;

/// Variants describing the shape of a object. The positions are relative to the objects
///
/// * `Pixel(position: Vector)`
/// * `Ellipse(center: Vector, a: u32, b: u32)`
/// * `Rect(center: Vector, width: u32, height: u32)`
/// * `Line(start: Vector, end: Vector)`
#[derive(PartialEq, Debug, Clone)]
pub enum Shape {
    ///Pixel(position: Vector)
    Pixel(Vector),
    ///Ellipse(center: Vector, a: u32, b: u32)
    Ellipse(Vector, u32, u32),
    ///Rect(center: Vector, width: u32, height: u32)
    Rect(Vector, u32, u32),
    ///Line(start: Vector, end: Vector)
    Line(Vector, Vector),
}

impl PartialEq<RefBody> for Body<Vector> {
    fn eq(&self, other: &RefBody) -> bool {
        other == self
    }
}
impl PartialEq<RefBody> for &Body<Vector> {
    fn eq(&self, other: &RefBody) -> bool {
        other == self
    }
}
impl PartialEq<&RefBody> for Body<Vector> {
    fn eq(&self, other: &&RefBody) -> bool {
        *other == self
    }
}
