use std::collections::HashSet;

use crate::physics_2d::{RefBody, Shape};
use crate::spatial_hashgrid::SpatialHashgrid;
use crate::PhysicalSize;
use uuid::Uuid;

type CollisionShape = Shape;

#[derive(PartialEq, Debug, Clone)]
pub struct CollisionBody {
    shape: CollisionShape,
    behaviour: Mass,
}

/// Variants describing the behaviour of a `CollisionBody`, when colliding with something
///
/// * `Infinite`: Acts as a wall with infinite mass. Basically absorbs all of the impacting bodies impulse and reflects double it (p2' = - p2)
/// * `Copy`: Copies the impacting bodies mass. Results in a simple impulse transfer between them (p1' = p2; p2' = p1)
/// * `Elastic(masss: f64)`: a finite mass of a elastically colliding body.
#[derive(PartialEq, Debug, Clone)]
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
#[derive(Debug)]
pub struct CollisionLayer {
    obstacles: HashSet<RefBody>,
    actors: HashSet<RefBody>,
    collision_grid: SpatialHashgrid<Uuid>,
}
///TODO: WHAT IS WITH BIG BODY SPANNING OVER MULTIPLE CELLS?
impl CollisionLayer {
    ///Creates a new CollisionLayer. The underlaying SpatialHashgrid will have the total dimensions `grid_size` and each cell in the grid has the dimensions `cell_size`
    pub fn new(grid_size: PhysicalSize<u32>, cell_size: PhysicalSize<u32>) -> Self {
        CollisionLayer {
            obstacles: HashSet::new(),
            actors: HashSet::new(),
            collision_grid: SpatialHashgrid::new(grid_size, cell_size),
        }
    }

    //Adds the `collision_body` to the layer according to `is_obstacle`.
    ///
    ///Returns whether the value was newly inserted. That is:
    ///
    ///If the underliying `SpatialHashgrid` did not previously contain this value, true is returned. If the grid already contained this value, false is returned and the body does not get added again.
    /// Meaning: If a body is allready part of the layer as a actor, it can not be added as a obstacle, before being removed and vice versa.
    pub fn add_body(&mut self, collision_body: RefBody, is_obstacle: bool) -> bool {
        let position = collision_body.position().clone();
        if self.collision_grid.insert(collision_body.id(), &position) {
            if is_obstacle {
                self.obstacles.insert(collision_body)
            } else {
                self.actors.insert(collision_body)
            };
            true
        } else {
            false
        }
    }

    ///Removes the body from the collision layer, if it is contained
    ///
    ///Return `true` if the body was part of the layer
    pub fn remove_body(&mut self, collision_body: &RefBody, is_obstacle: Option<bool>) -> bool {
        if self
            .collision_grid
            .contains(&collision_body.id(), &collision_body.position())
        {
            let body_in_layer = match is_obstacle {
                Some(is_obstacle) => {
                    if is_obstacle {
                        self.obstacles.remove(collision_body)
                    } else {
                        self.actors.remove(collision_body)
                    }
                }
                None => {
                    if !self.obstacles.remove(collision_body) {
                        self.actors.remove(collision_body)
                    } else {
                        true
                    }
                }
            };
            if body_in_layer {
                self.collision_grid
                    .remove(&collision_body.id(), &collision_body.position());
            }
            body_in_layer
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math_2d::Vector;

    use super::*;

    #[test]
    fn add_to_layer() {
        let b1 = RefBody::new(1.0, Vector::zero(), Vector::zero(), 0.0, 0.0, None, None);
        let b2 = RefBody::new(
            2.0,
            Vector::scalar(20.0),
            Vector::zero(),
            0.0,
            0.0,
            None,
            None,
        );
        let b3 = RefBody::new(
            3.0,
            Vector::scalar(100.0),
            Vector::zero(),
            0.0,
            0.0,
            None,
            None,
        );
        let b4 = RefBody::new(
            4.0,
            Vector::new(0.0, 20.0),
            Vector::zero(),
            0.0,
            0.0,
            None,
            None,
        );

        let mut collision_layer =
            CollisionLayer::new(PhysicalSize::new(100, 100), PhysicalSize::new(10, 10));
        assert!(collision_layer.add_body(b1.clone(), true));
        assert!(collision_layer.add_body(b2.clone(), false));
        assert!(collision_layer.add_body(b3.clone(), true));
        assert!(collision_layer.add_body(b4.clone(), false));
        assert!(!collision_layer.add_body(b3.clone(), false));

        println!("{:?}", collision_layer);
        assert!(!collision_layer.remove_body(&b3, Some(false)));
        assert!(collision_layer.remove_body(&b3, Some(true)));
        assert!(!collision_layer.remove_body(&b3, Some(false)));
        assert!(!collision_layer.remove_body(&b3, Some(true)));
        assert!(collision_layer.remove_body(&b2, Some(false)));
        assert!(!collision_layer.remove_body(&b2, None));
        assert!(collision_layer.remove_body(&b1, None));
        assert!(collision_layer.remove_body(&b4, None));
    }
}
