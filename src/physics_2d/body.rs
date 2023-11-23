use std::hash::Hash;
use uuid::Uuid;

use super::{collision::CollisionBody, Shape};

pub type VisiualShape = Shape;

#[derive(Debug, Clone)]
pub struct Body<T> {
    pub mass: f64,
    pub position: T,
    pub velocity: T,
    pub angle_deg: f64,
    pub rotation_velocity: f64,
    shape: Option<VisiualShape>,
    collision_body: Option<CollisionBody>,
    uuid: Uuid,
}
impl<T> Body<T> {
    pub fn new(
        mass: f64,
        position: T,
        velocity: T,
        angle_deg: f64,
        rotation_velocity: f64,
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
impl<T> PartialEq for Body<T> {
    fn eq(&self, other: &Body<T>) -> bool {
        self.uuid == other.uuid
    }
}
impl<T> PartialEq<&Body<T>> for Body<T> {
    fn eq(&self, other: &&Body<T>) -> bool {
        self.uuid == other.uuid
    }
}
impl<T> PartialEq<Body<T>> for &Body<T> {
    fn eq(&self, other: &Body<T>) -> bool {
        self.uuid == other.uuid
    }
}
impl<T> Eq for Body<T> {}
impl<T> Hash for Body<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::math_2d::Vector;

    use super::*;

    #[test]
    fn bodies_equal() {
        let b1 = Body::new(0.0, Vector::zero(), Vector::zero(), 0.0, 0.0, None, None);
        let b2 = Body::new(0.0, Vector::zero(), Vector::zero(), 0.0, 0.0, None, None);
        let mut b3 = Body::new(0.0, Vector::zero(), Vector::zero(), 0.0, 0.0, None, None);
        b3.uuid = b1.uuid;

        assert_eq!(&b2, b2);
        assert_eq!(b2, b2);
        assert_eq!(b2, &b2);
        assert_eq!(&b2, &b2);
        assert_ne!(b1, b2);
        assert_eq!(&b1, b3);
        assert_eq!(b1, b3);
        assert_eq!(b1, &b3);
        assert_eq!(&b1, &b3);
    }
}
