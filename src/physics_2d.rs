mod body;
pub use body::Body;

mod collision;
pub use collision::CollisionLayer;
use collision::{CollisionBody, Mass};

use crate::math_2d::Vector;

/// Variants describing the shape of a object. The positions are relative to the objects
///
/// * `Pixel(position: Vector)`
/// * `Ellipse(center: Vector, a: u32, b: u32)`
/// * `Rect(center: Vector, width: u32, height: u32)`
/// * `Line(start: Vector, end: Vector)`
#[derive(PartialEq, Debug)]
pub enum Shape {
    Pixel(Vector),             //Pixel(position: Vector)
    Ellipse(Vector, u32, u32), //Ellipse(center: Vector, a: u32, b: u32)
    Rect(Vector, u32, u32),    //Rect(origin: Vector, width: u32, height: u32)
    Line(Vector, Vector),      //Line(start: Vector, end: Vector)
}
