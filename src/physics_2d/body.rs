use std::hash::Hash;
use uuid::Uuid;

use crate::renderer::Renderer;

use super::{collision::CollisionBody, Shape};

#[derive(Debug, Clone)]
pub struct VisualShape {
    shape: Shape,
    fill: bool,
}
impl VisualShape {
    fn new(shape: Shape, fill: bool) -> Self {
        VisualShape { shape, fill }
    }

    fn render(&self, renderer: &mut Renderer, angle_deg: f64) {
        if self.fill {
            self.fill(renderer, angle_deg)
        } else {
            self.draw(renderer, angle_deg)
        }
    }

    fn fill(&self, renderer: &mut Renderer, angle_deg: f64) {
        match &self.shape {
            Shape::Line(start, end) => {
                let center = (start + end) / 2.0;
                renderer.fill_line(
                    &start.rotate_degree_around(angle_deg, &center),
                    &end.rotate_degree_around(angle_deg, &center),
                )
            }
            Shape::Pixel(pos) => renderer.fill_pixel(&pos),
            Shape::Rect(center, width, height) => {
                renderer.fill_rect(&center, *width, *height, angle_deg)
            }
            Shape::Ellipse(center, a, b) => renderer.fill_ellipse(&center, *a, *b, angle_deg),
        }
    }

    fn draw(&self, renderer: &mut Renderer, angle_deg: f64) {
        match &self.shape {
            Shape::Line(start, end) => {
                let center = (start + end) / 2.0;
                renderer.draw_line(
                    &start.rotate_degree_around(angle_deg, &center),
                    &end.rotate_degree_around(angle_deg, &center),
                )
            }
            Shape::Pixel(pos) => renderer.draw_pixel(&pos),
            Shape::Rect(center, width, height) => {
                renderer.draw_rect(&center, *width, *height, angle_deg)
            }
            Shape::Ellipse(center, a, b) => renderer.draw_ellipse(&center, *a, *b, angle_deg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Body<T> {
    pub mass: f64,
    pub position: T,
    pub velocity: T,
    pub angle_deg: f64,
    pub rotation_velocity: f64,
    shape: Option<VisualShape>,
    collision_body: Option<CollisionBody>,
    pub uuid: Uuid,
}
impl<T> Body<T> {
    pub fn new(
        mass: f64,
        position: T,
        velocity: T,
        angle_deg: f64,
        rotation_velocity: f64,
        shape: Option<VisualShape>,
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

    pub fn render(&self, renderer: &mut Renderer) {
        if let Some(shape) = &self.shape {
            shape.render(renderer, self.angle_deg)
        }
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

pub struct BodyBuilder<T> {
    pub mass: f64,
    pub position: T,
    pub velocity: T,
    pub angle_deg: f64,
    pub rotation_velocity: f64,
    shape: Option<VisualShape>,
    collision_body: Option<CollisionBody>,
}
impl<T: Default> BodyBuilder<T> {
    pub fn new() -> Self {
        BodyBuilder {
            mass: 0.0,
            position: T::default(),
            velocity: T::default(),
            angle_deg: 0.0,
            rotation_velocity: 0.0,
            shape: None,
            collision_body: None,
        }
    }
}
impl<T: Clone> BodyBuilder<T> {
    pub fn new_with_default(default: T) -> Self {
        BodyBuilder {
            mass: 0.0,
            position: default.clone(),
            velocity: default.clone(),
            angle_deg: 0.0,
            rotation_velocity: 0.0,
            shape: None,
            collision_body: None,
        }
    }
}
impl<T> BodyBuilder<T> {
    pub fn mass(mut self, mass: f64) -> Self {
        self.mass = mass;
        self
    }

    pub fn position(mut self, position: T) -> Self {
        self.position = position;
        self
    }

    pub fn velocity(mut self, velocity: T) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn angle_deg(mut self, angle_deg: f64) -> Self {
        self.angle_deg = angle_deg;
        self
    }

    pub fn rotation_velocity(mut self, velocity: f64) -> Self {
        self.rotation_velocity = velocity;
        self
    }

    pub fn shape(mut self, shape: Shape, fill: bool) -> Self {
        self.shape = Some(VisualShape::new(shape, fill));
        self
    }

    pub fn collision(mut self, collision_body: CollisionBody) -> Self {
        self.collision_body = Some(collision_body);
        self
    }

    pub fn build(self) -> Body<T> {
        Body::new(
            self.mass,
            self.position,
            self.velocity,
            self.angle_deg,
            self.rotation_velocity,
            self.shape,
            self.collision_body,
        )
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
