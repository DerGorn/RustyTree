use std::ops::Deref;
use std::{cell::RefCell, hash::Hash, rc::Rc};

use crate::math_2d::Vector;

use super::{Body, CollisionBody, VisiualShape};

#[derive(Debug, Clone, Eq)]
pub struct RefBody(pub Rc<RefCell<Body<Vector>>>);

impl RefBody {
    pub fn new(
        mass: f64,
        position: Vector,
        velocity: Vector,
        angle_deg: f64,
        rotation_velocity: f64,
        shape: Option<VisiualShape>,
        collision_body: Option<CollisionBody>,
    ) -> Self {
        Body::new(
            mass,
            position,
            velocity,
            angle_deg,
            rotation_velocity,
            shape,
            collision_body,
        )
        .into()
    }

    pub fn mass(&self) -> f64 {
        self.borrow().mass
    }
    pub fn angle_deg(&self) -> f64 {
        self.borrow().angle_deg
    }
    pub fn rotation_velocity(&self) -> f64 {
        self.borrow().rotation_velocity
    }
    pub fn position(&self) -> Vector {
        self.borrow().position.clone()
    }
    pub fn velocity(&self) -> Vector {
        self.borrow().velocity.clone()
    }

    pub fn has_collision(&self) -> bool {
        self.borrow().has_collision()
    }
}

impl PartialEq for RefBody {
    fn eq(&self, other: &Self) -> bool {
        *self.0.borrow() == *other.0.borrow()
    }
}
impl PartialEq<Body<Vector>> for RefBody {
    fn eq(&self, other: &Body<Vector>) -> bool {
        *self.0.borrow() == other
    }
}
impl PartialEq<&Body<Vector>> for RefBody {
    fn eq(&self, other: &&Body<Vector>) -> bool {
        &*self.0.borrow() == other
    }
}

impl From<Body<Vector>> for RefBody {
    fn from(val: Body<Vector>) -> Self {
        RefBody(Rc::new(RefCell::new(val)))
    }
}

impl From<Rc<RefCell<Body<Vector>>>> for RefBody {
    fn from(val: Rc<RefCell<Body<Vector>>>) -> Self {
        RefBody(val)
    }
}

impl Deref for RefBody {
    type Target = RefCell<Body<Vector>>;

    fn deref(&self) -> &RefCell<Body<Vector>> {
        &self.0
    }
}

impl Hash for RefBody {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ref_body() {
        let mut b1 = Body::new(0.0, Vector::zero(), Vector::zero(), 0.0, 0.0, None, None);
        let b2 = Body::new(0.0, Vector::zero(), Vector::zero(), 0.0, 0.0, None, None);

        let ref_b1 = RefBody::from(b1.clone());

        assert_eq!(b1, ref_b1);
        assert_ne!(b2, ref_b1);
        assert_eq!(&b1, &ref_b1);
        assert_ne!(b2, &ref_b1);
        assert_eq!(&b1, ref_b1);

        b1.angle_deg = 100.0;
        assert_eq!(b1, ref_b1);

        let ref_b1_2 = ref_b1.clone();
        let ref_b1_3 = RefBody::from(b1.clone());

        assert_eq!(ref_b1, ref_b1_2);
        assert_eq!(ref_b1, ref_b1_3);
        assert_eq!(&ref_b1, &ref_b1_3);
    }
}
