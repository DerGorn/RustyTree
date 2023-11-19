use crate::{math_2d::Vector, spatial_hashgrid::SpatialHashgrid};
use std::{hash::Hash, rc::Rc};
use uuid::Uuid;
use crate::PhysicalSize;
/// Variants describing the shape of a object. The positions are relative to the objects
///
/// * `Pixel(position: Vector)`
/// * `Ellipse(center: Vector, a: u32, b: u32)`
/// * `Rect(center: Vector, width: u32, height: u32)`
/// * `Line(start: Vector, end: Vector)`
#[derive(PartialEq)]
pub enum Shape {
    Pixel(Vector),             //Pixel(position: Vector)
    Ellipse(Vector, u32, u32), //Ellipse(center: Vector, a: u32, b: u32)
    Rect(Vector, u32, u32),    //Rect(origin: Vector, width: u32, height: u32)
    Line(Vector, Vector),      //Line(start: Vector, end: Vector)
}

type CollisionShape = Shape;
type VisiualShape = Shape;

#[derive(PartialEq)]
pub struct CollisionBody {
    shape: CollisionShape,
    behaviour: Mass,
}

/// Variants describing the behaviour of a `CollisionBody`, when colliding with something
///
/// * `Infinite`: Acts as a wall with infinite mass. Basically absorbs all of the impacting bodies impulse and reflects double it (p2' = - p2)
/// * `Copy`: Copies the impacting bodies mass. Results in a simple impulse transfer between them (p1' = p2; p2' = p1)
/// * `Elastic(masss: f64)`: a finite mass of a elastically colliding body.
#[derive(PartialEq)]
pub enum Mass {
    Infinite,
    Copy,
    Elastic(f64),
}

pub struct Body {
    pub mass: f64,
    pub position: Vector,
    pub velocity: Vector,
    pub angle_deg: f64,
    pub rotation_velocity: Vector,
    shape: Option<VisiualShape>,
    collision_body: Option<CollisionBody>,
    uuid: Uuid,
}
impl Body {
    pub fn new(
        mass: f64,
        position: Vector,
        velocity: Vector,
        angle_deg: f64,
        rotation_velocity: Vector,
        shape: Option<VisiualShape>,
        collision_body: Option<CollisionBody>,
    ) -> Self {
        Body {
            mass,
            position,
            velocity,
            angle_deg,
            rotation_velocity,
            shape,
            collision_body,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn has_collision(&self) -> bool {
        self.collision_body.is_some()
    }
}
impl PartialEq for Body {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
impl Eq for Body {}
impl Hash for Body {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

/// An area in which collidable objects are grouped together.
///
/// A `CollisonLayer` has two groups:
/// * `obstacles`: `CollisionBody`es that collide with everything in the layer
/// * `actors`: only collide with `obstacles`, but not other `actors`
///
pub struct CollisionLayer {
    obstacles: Vec<Rc<Body>>,
    actors: Vec<Rc<Body>>,
    collision_grid: SpatialHashgrid<Body>,
}
impl CollisionLayer {
    pub fn new(grid_size: PhysicalSize<u32>, cell_size: PhysicalSize<u32>) -> Self {
        CollisionLayer {
            obstacles: vec![],
            actors: vec![],
            collision_grid: SpatialHashgrid::new(grid_size, cell_size),
        }
    }

    pub fn add_body(&mut self, collision_body: Rc<Body>, is_obstacle: bool) {
        let position = collision_body.position.clone();
        self.collision_grid
            .insert(collision_body.clone(), &position);
        if is_obstacle {
            self.obstacles.push(collision_body)
        } else {
            self.actors.push(collision_body)
        }
    }

    ///Removes the body from the collision layer, if it is contained
    ///
    ///Return `true` if the body was part of the layer
    pub fn remove_body(&mut self, collision_body: &Body, is_obstacle: Option<bool>) -> bool {
        if self
            .collision_grid
            .remove(collision_body, &collision_body.position)
        {
            match is_obstacle {
                Some(is_obstacle) => {
                    if is_obstacle {
                        match self
                            .obstacles
                            .iter()
                            .position(|el| el.as_ref() == collision_body)
                        {
                            None => {
                                return false;
                            }
                            Some(index) => {
                                self.obstacles.remove(index);
                            }
                        }
                    } else {
                        match self
                            .actors
                            .iter()
                            .position(|el| el.as_ref() == collision_body)
                        {
                            None => {
                                return false;
                            }
                            Some(index) => {
                                self.actors.remove(index);
                            }
                        }
                    }
                }
                None => match self
                    .obstacles
                    .iter()
                    .position(|el| el.as_ref() == collision_body)
                {
                    None => match self
                        .actors
                        .iter()
                        .position(|el| el.as_ref() == collision_body)
                    {
                        None => {
                            return false;
                        }
                        Some(index) => {
                            self.actors.remove(index);
                        }
                    },
                    Some(index) => {
                        self.obstacles.remove(index);
                    }
                },
            }
            true
        } else {
            false
        }
    }
}
