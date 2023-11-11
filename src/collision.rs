use crate::Vector;
/// Variants describing the shape of a object with collision
///
/// * `Pixel(position: Vector)`
/// * `Ellipse(center: Vector, a: u32, b: u32)`
/// * `Rect(origin: Vector, width: u32, height: u32)`
/// * `Line(start: Vector, end: Vector)`
pub enum CollisionShape {
    Pixel(Vector),             //Pixel(position: Vector)
    Ellipse(Vector, u32, u32), //Ellipse(center: Vector, a: u32, b: u32)
    Rect(Vector, u32, u32),    //Rect(origin: Vector, width: u32, height: u32)
    Line(Vector, Vector),      //Line(start: Vector, end: Vector)
}

pub struct CollisionBody {
    shape: CollisionShape,
    behaviour: Mass,
}

/// Variants describing the behaviour of a `CollisionBody`, when colliding with something
///
/// * `Infinite`: Acts as a wall with infinite mass. Basically absorbs all of the impacting bodies impulse and reflects double it (p2' = - p2)
/// * `Copy`: Copies the impacting bodies mass. Results in a simple impulse transfer between them (p1' = p2; p2' = p1)
/// * `Elastic(masss: f64)`: a finite mass of a elastically colliding body. 
pub enum Mass {
    Infinite,
    Copy,
    Elastic(f64),
}

/// An area in which collidable objects are grouped together.
///
/// A `CollisonLayer` has two groups:
/// * `obstacles`: `CollisionBody`es that collide with everything in the layer
/// * `actors`: only collide with `obstacles`, but not other `actors`
///
pub struct CollisionLayer<'a> {
    obstacles: Vec<&'a CollisionBody>,
    actors: Vec<&'a CollisionBody>,
}
