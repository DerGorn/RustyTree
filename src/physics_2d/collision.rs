use std::collections::HashSet;

use crate::math_2d::Vector;
use crate::physics_2d::{RefBody, Shape};
use crate::spatial_hashgrid::SpatialHashgrid;
use crate::PhysicalSize;
use uuid::Uuid;

///`Rect` and `Ellipse` are considered to be hollow for collision
type CollisionShape = Shape;
impl CollisionShape {
    fn intersect_line_line(
        l1: &CollisionShape,
        l1_pos: &Vector,
        l1_degree: f64,
        l2: &CollisionShape,
        l2_pos: &Vector,
        l2_degree: f64,
    ) -> Vec<Vector> {
        match l1 {
            CollisionShape::Line(start, end) => {
                let center = (start + end) / 2.0;
                let start = start.rotate_degree_around(l1_degree, &center);
                let end = end.rotate_degree_around(l1_degree, &center);
                let v1 = end - &start;

                let support_base = l2_pos - l1_pos - start;
                match l2 {
                    CollisionShape::Line(other_start, other_end) => {
                        let center = (other_start + other_end) / 2.0;
                        let other_start = other_start.rotate_degree_around(l2_degree, &center);
                        let other_end = other_end.rotate_degree_around(l2_degree, &center);
                        let support = &other_start + support_base;
                        let v2 = other_end - other_start;
                        let intersect = v1.intersection(&v2, &support);
                        v1.get_intersection_point(intersect)
                    }
                    x => unreachable!(
                        "Only Expects 'Line' in 'intersect_line_line', but encountered {:?}",
                        x
                    ),
                }
            }
            x => unreachable!(
                "Only Expects 'Line' in 'intersect_line_line', but encountered {:?}",
                x
            ),
        }
    }
    fn intersect_line_pixel(
        l: &CollisionShape,
        l_pos: &Vector,
        l_degree: f64,
        p: &CollisionShape,
        p_pos: &Vector,
    ) -> Vec<Vector> {
        match l {
            CollisionShape::Line(start, end) => {
                let center = (start + end) / 2.0;
                let start = start.rotate_degree_around(l_degree, &center);
                let end = end.rotate_degree_around(l_degree, &center);
                let v1 = end - &start;

                let support_base = p_pos - l_pos - start;
                match p {
                    Self::Pixel(pos) => {
                        let support = pos + support_base;
                        let intersect = v1.intersection(&Vector::zero(), &support);
                        v1.get_intersection_point(intersect)
                    }
                    x => unreachable!(
                        "Only Expects 'Pixel' in 'intersect_line_pixel', but encountered {:?}",
                        x
                    ),
                }
            }
            x => unreachable!(
                "Only Expects 'Line' in 'intersect_line_pixel', but encountered {:?}",
                x
            ),
        }
    }
    fn intersect_line_rect(
        l: &CollisionShape,
        l_pos: &Vector,
        l_degree: f64,
        r: &CollisionShape,
        r_pos: &Vector,
        r_degree: f64,
    ) -> Vec<Vector> {
        match l {
            CollisionShape::Line(start, end) => {
                let center = (start + end) / 2.0;
                let start = start.rotate_degree_around(l_degree, &center);
                let end = end.rotate_degree_around(l_degree, &center);
                let v1 = end - &start;

                let support_base = r_pos - l_pos - start;
                match r {
                    Self::Rect(center, width, height) => {
                        let main_diagonal = Vector::new(*width as f64, *height as f64);
                        let off_diagonal = Vector::new(-(*width as f64), *height as f64);

                        //Basic Corners
                        let bottom_right = (center + &main_diagonal).rotate_degree(r_degree);
                        let top_left = (center - main_diagonal).rotate_degree(r_degree);
                        let bottom_left = (center + &off_diagonal).rotate_degree(r_degree);
                        let top_right = (center - off_diagonal).rotate_degree(r_degree);

                        let support = &bottom_left + &support_base;
                        let mut intersection_points = vec![];

                        let bottom_intersection =
                            v1.intersection(&(&bottom_right - &bottom_left), &support);
                        intersection_points
                            .append(&mut v1.get_intersection_point(bottom_intersection));
                        let left_intersection =
                            v1.intersection(&(&top_left - &bottom_left), &support);
                        intersection_points
                            .append(&mut v1.get_intersection_point(left_intersection));

                        let support = &top_right + &support_base;

                        let top_intersection = v1.intersection(&(&top_left - &top_right), &support);
                        intersection_points
                            .append(&mut v1.get_intersection_point(top_intersection));
                        let right_intersection =
                            v1.intersection(&(&bottom_right - &top_right), &support);
                        intersection_points
                            .append(&mut v1.get_intersection_point(right_intersection));
                        intersection_points
                    }
                    x => unreachable!(
                        "Only Expects 'Rect' in 'intersect_line_rect', but encountered {:?}",
                        x
                    ),
                }
            }
            x => unreachable!(
                "Only Expects 'Line' in 'intersect_line_rect', but encountered {:?}",
                x
            ),
        }
    }
    fn intersect_line_ellipse(
        l: &CollisionShape,
        l_pos: &Vector,
        l_degree: f64,
        e: &CollisionShape,
        e_pos: &Vector,
        e_degree: f64,
    ) -> Vec<Vector> {
        match l {
            CollisionShape::Line(start, end) => {
                let center = (start + end) / 2.0;
                let start = start.rotate_degree_around(l_degree, &center);
                let end = end.rotate_degree_around(l_degree, &center);
                let v1 = end - &start;

                let support_base = e_pos - l_pos - start;
                match e {
                    Self::Ellipse(center, a, b) => {
                        let center = center + support_base;
                        let mut a = *a as f64;
                        let mut b = *b as f64;
                        let mut alpha = e_degree;

                        if a < b {
                            (a, b) = (b, a);
                            alpha -= 90.0;
                        }

                        let focal_distance = (a.powi(2) - b.powi(2)).sqrt();
                        let focal_offset = if alpha != e_degree {
                            Vector::new(0.0, focal_distance)
                        } else {
                            Vector::new(focal_distance, 0.0)
                        }
                        .rotate_degree(e_degree);

                        let f1 = &center + &focal_offset;
                        let f2 = &center - &focal_offset;

                        let point_on_ellipse = center + Vector::new(0.0, b).rotate_degree(alpha);
                        let distance =
                            point_on_ellipse.distance(&f1) + point_on_ellipse.distance(&f2);

                        let calc_distance = |vector: &Vector| {
                            vector.distance(&f1) + vector.distance(&f2) - distance
                        };

                        let mut intersections = vec![];
                        let mut open_intervalls = vec![(Vector::zero(), v1)];
                        while let Some((start, end)) = open_intervalls.pop() {
                            let edge = &end - &start;
                            let step = &edge / edge.length();
                            let center = &edge / 2.0;
                            let left = calc_distance(&(&center - &step));
                            let right = calc_distance(&(&center + &step));
                            let distance = calc_distance(&center);

                            if left < distance {
                                open_intervalls.push((start, center.clone()));
                            }
                            if right < distance {
                                open_intervalls.push((center.clone(), end));
                            }
                            if distance < 1e-6 {
                                intersections.push(center);
                            }
                        }

                        intersections
                    }
                    x => unreachable!(
                        "Only Expects 'Ellipse' in 'intersect_line_ellipse', but encountered {:?}",
                        x
                    ),
                }
            }
            x => unreachable!(
                "Only Expects 'Line' in 'intersect_line_ellipse', but encountered {:?}",
                x
            ),
        }
    }
    fn intersect_pixel_pixel(
        p1: &CollisionShape,
        p1_pos: &Vector,
        p2: &CollisionShape,
        p2_pos: &Vector,
    ) -> Vec<Vector> {
        match p1 {
            CollisionShape::Pixel(pos1) => {
                let pos1 = pos1 + p1_pos;
                match p2 {
                    Self::Pixel(pos2) => {
                        if pos1 == pos2 + p2_pos {
                            vec![pos1]
                        } else {
                            vec![]
                        }
                    }
                    x => unreachable!(
                        "Only Expects 'Pixel' in 'intersect_pixel_pixel', but encountered {:?}",
                        x
                    ),
                }
            }
            x => unreachable!(
                "Only Expects 'Pixel' in 'intersect_pixel_pixel', but encountered {:?}",
                x
            ),
        }
    }
    fn intersect_pixel_rect(
        p: &CollisionShape,
        p_pos: &Vector,
        r: &CollisionShape,
        r_pos: &Vector,
        r_degree: f64,
    ) -> Vec<Vector> {
        match p {
            CollisionShape::Pixel(pos) => {
                let pos = pos + p_pos;
                match r {
                    Self::Rect(center, width, height) => {
                        let width = *width as f64;
                        let height = *height as f64;
                        let center = r_pos + center;
                        let p_rot = pos.rotate_degree_around(-r_degree, &center);
                        if (((p_rot.x - center.x + width).abs() < 1e-6
                            || (p_rot.x - center.x - width).abs() < 1e-6)
                            && p_rot.y > center.y - height
                            && p_rot.y < center.y + height)
                            || (((p_rot.y - center.y + height).abs() < 1e-6
                                || (p_rot.y - center.y - height).abs() < 1e-6)
                                && p_rot.x > center.x - width
                                && p_rot.x < center.x + width)
                        {
                            vec![p_rot]
                        } else {
                            vec![]
                        }
                    }
                    x => unreachable!(
                        "Only Expects 'Rect' in 'intersect_pixel_rect', but encountered {:?}",
                        x
                    ),
                }
            }
            x => unreachable!(
                "Only Expects 'Pixel' in 'intersect_pixel_rect', but encountered {:?}",
                x
            ),
        }
    }
    fn intersect_pixel_ellipse(
        p: &CollisionShape,
        p_pos: &Vector,
        e: &CollisionShape,
        e_pos: &Vector,
        e_degree: f64,
    ) -> Vec<Vector> {
        match p {
            CollisionShape::Pixel(pos) => {
                let pos = pos + p_pos;
                let support_base = e_pos - &pos;
                match e {
                    Self::Ellipse(center, a, b) => {
                        let center = center + support_base;
                        let mut a = *a as f64;
                        let mut b = *b as f64;
                        let mut alpha = e_degree;

                        if a < b {
                            (a, b) = (b, a);
                            alpha -= 90.0;
                        }

                        let focal_distance = (a.powi(2) - b.powi(2)).sqrt();
                        let focal_offset = if alpha != e_degree {
                            Vector::new(0.0, focal_distance)
                        } else {
                            Vector::new(focal_distance, 0.0)
                        }
                        .rotate_degree(e_degree);

                        let f1 = &center + &focal_offset;
                        let f2 = &center - &focal_offset;

                        let point_on_ellipse = &center + Vector::new(0.0, b).rotate_degree(alpha);
                        let distance =
                            point_on_ellipse.distance(&f1) + point_on_ellipse.distance(&f2);

                        let p_rot = pos.rotate_degree_around(-e_degree, &center);
                        let distance = p_rot.distance(&f1) + p_rot.distance(&f2) - distance;
                        if distance.abs() < 1e-6 {
                            vec![p_rot]
                        } else {
                            vec![]
                        }
                    }
                    x => unreachable!(
                        "Only Expects 'Ellipse' in 'intersect_pixel_ellipse', but encountered {:?}",
                        x
                    ),
                }
            }
            x => unreachable!(
                "Only Expects 'Pixel' in 'intersect_pixel_pixel', but encountered {:?}",
                x
            ),
        }
    }
    fn intersect_rect_rect(
        r1: &CollisionShape,
        r1_pos: &Vector,
        r1_degree: f64,
        r2: &CollisionShape,
        r2_pos: &Vector,
        r2_degree: f64,
    ) -> Vec<Vector> {
        match r1 {
            CollisionShape::Rect(center, width, height) => {
                let main_diagonal = Vector::new(*width as f64, *height as f64);
                let off_diagonal = Vector::new(-(*width as f64), *height as f64);

                //Basic Corners
                let bottom_right = (center + &main_diagonal).rotate_degree(r1_degree);
                let top_left = (center - main_diagonal).rotate_degree(r1_degree);
                let bottom_left = (center + &off_diagonal).rotate_degree(r1_degree);
                let top_right = (center - off_diagonal).rotate_degree(r1_degree);
                match r2 {
                    Self::Rect(..) => {
                        let mut intersections = vec![];
                        intersections.append(&mut CollisionShape::intersect_line_rect(
                            &Shape::Line(Vector::zero(), bottom_left.clone()),
                            &bottom_right,
                            0.0,
                            r2,
                            r2_pos,
                            r2_degree,
                        ));
                        intersections.append(&mut CollisionShape::intersect_line_rect(
                            &Shape::Line(Vector::zero(), top_right.clone()),
                            &bottom_right,
                            0.0,
                            r2,
                            r2_pos,
                            r2_degree,
                        ));
                        intersections.append(&mut CollisionShape::intersect_line_rect(
                            &Shape::Line(Vector::zero(), bottom_left.clone()),
                            &top_left,
                            0.0,
                            r2,
                            r2_pos,
                            r2_degree,
                        ));
                        intersections.append(&mut CollisionShape::intersect_line_rect(
                            &Shape::Line(Vector::zero(), top_right.clone()),
                            &top_left,
                            0.0,
                            r2,
                            r2_pos,
                            r2_degree,
                        ));
                        intersections
                    }
                    x => unreachable!(
                        "Only Expects 'Rect' in 'intersect_rect_rect', but encountered {:?}",
                        x
                    ),
                }
            }
            x => unreachable!(
                "Only Expects 'Rect' in 'intersect_rect_rect', but encountered {:?}",
                x
            ),
        }
    }
    fn intersect_rect_ellipse(
        r: &CollisionShape,
        r_pos: &Vector,
        r_degree: f64,
        e: &CollisionShape,
        e_pos: &Vector,
        e_degree: f64,
    ) -> Vec<Vector> {
        match r {
            CollisionShape::Rect(center, width, height) => {
                let main_diagonal = Vector::new(*width as f64, *height as f64);
                let off_diagonal = Vector::new(-(*width as f64), *height as f64);

                //Basic Corners
                let bottom_right = (center + &main_diagonal).rotate_degree(r_degree);
                let top_left = (center - main_diagonal).rotate_degree(r_degree);
                let bottom_left = (center + &off_diagonal).rotate_degree(r_degree);
                let top_right = (center - off_diagonal).rotate_degree(r_degree);
                match e {
                    Self::Ellipse(..) => {
                        let mut intersections = vec![];
                        intersections.append(&mut CollisionShape::intersect_line_ellipse(
                            &Shape::Line(Vector::zero(), bottom_left.clone()),
                            &bottom_right,
                            0.0,
                            e,
                            e_pos,
                            e_degree,
                        ));
                        intersections.append(&mut CollisionShape::intersect_line_ellipse(
                            &Shape::Line(Vector::zero(), top_right.clone()),
                            &bottom_right,
                            0.0,
                            e,
                            e_pos,
                            e_degree,
                        ));
                        intersections.append(&mut CollisionShape::intersect_line_ellipse(
                            &Shape::Line(Vector::zero(), bottom_left.clone()),
                            &top_left,
                            0.0,
                            e,
                            e_pos,
                            e_degree,
                        ));
                        intersections.append(&mut CollisionShape::intersect_line_ellipse(
                            &Shape::Line(Vector::zero(), top_right.clone()),
                            &top_left,
                            0.0,
                            e,
                            e_pos,
                            e_degree,
                        ));
                        intersections
                    }
                    x => unreachable!(
                        "Only Expects 'Rect' in 'intersect_rect_ellipse', but encountered {:?}",
                        x
                    ),
                }
            }
            x => unreachable!(
                "Only Expects 'Rect' in 'intersect_rect_ellipse', but encountered {:?}",
                x
            ),
        }
    }
    fn intersect_ellipse_ellipse(
        e1: &CollisionShape,
        e1_pos: &Vector,
        e1_degree: f64,
        e2: &CollisionShape,
        e2_pos: &Vector,
        e2_degree: f64,
    ) -> Vec<Vector> {
        match e1 {
            CollisionShape::Ellipse(c1, a1, b1) => match e2 {
                Self::Ellipse(c2, a2, b2) => {
                    let mut intersections = vec![];
                    intersections;
                    todo!("ELI ELI STUFF")
                }
                x => unreachable!(
                    "Only Expects 'Ellipse' in 'intersect_ellipse_ellipse', but encountered {:?}",
                    x
                ),
            },
            x => unreachable!(
                "Only Expects 'Ellipse' in 'intersect_ellipse_ellipse', but encountered {:?}",
                x
            ),
        }
    }

    fn intersection(
        &self,
        self_pos: &Vector,
        self_degree: f64,
        other: &CollisionShape,
        other_pos: &Vector,
        other_degree: f64,
    ) -> Option<Vec<Vector>> {
        let intersection_points = match self {
            Self::Line(..) => match other {
                Self::Line(..) => CollisionShape::intersect_line_line(
                    self,
                    self_pos,
                    self_degree,
                    other,
                    other_pos,
                    other_degree,
                ),
                Self::Pixel(..) => CollisionShape::intersect_line_pixel(
                    self,
                    self_pos,
                    self_degree,
                    other,
                    other_pos,
                ),
                Self::Rect(..) => CollisionShape::intersect_line_rect(
                    self,
                    self_pos,
                    self_degree,
                    other,
                    other_pos,
                    other_degree,
                ),
                Self::Ellipse(..) => CollisionShape::intersect_line_ellipse(
                    self,
                    self_pos,
                    self_degree,
                    other,
                    other_pos,
                    other_degree,
                ),
            },
            Self::Pixel(..) => match other {
                Self::Line(..) => CollisionShape::intersect_line_pixel(
                    other,
                    other_pos,
                    other_degree,
                    self,
                    self_pos,
                ),
                Self::Pixel(..) => {
                    CollisionShape::intersect_pixel_pixel(self, self_pos, other, other_pos)
                }
                Self::Rect(..) => CollisionShape::intersect_pixel_rect(
                    self,
                    self_pos,
                    other,
                    other_pos,
                    other_degree,
                ),
                Self::Ellipse(..) => CollisionShape::intersect_pixel_ellipse(
                    self,
                    self_pos,
                    other,
                    other_pos,
                    other_degree,
                ),
            },
            Self::Rect(..) => match other {
                Self::Line(..) => CollisionShape::intersect_line_rect(
                    other,
                    other_pos,
                    other_degree,
                    self,
                    self_pos,
                    self_degree,
                ),
                Self::Pixel(..) => CollisionShape::intersect_pixel_rect(
                    other,
                    other_pos,
                    self,
                    self_pos,
                    self_degree,
                ),
                Self::Rect(..) => CollisionShape::intersect_rect_rect(
                    self,
                    self_pos,
                    self_degree,
                    other,
                    other_pos,
                    other_degree,
                ),
                Self::Ellipse(..) => CollisionShape::intersect_rect_ellipse(
                    self,
                    self_pos,
                    self_degree,
                    other,
                    other_pos,
                    other_degree,
                ),
            },
            Self::Ellipse(..) => match other {
                Self::Line(..) => CollisionShape::intersect_line_ellipse(
                    other,
                    other_pos,
                    other_degree,
                    self,
                    self_pos,
                    self_degree,
                ),
                Self::Pixel(..) => CollisionShape::intersect_pixel_ellipse(
                    other,
                    other_pos,
                    self,
                    self_pos,
                    self_degree,
                ),
                Self::Rect(..) => CollisionShape::intersect_rect_ellipse(
                    other,
                    other_pos,
                    other_degree,
                    self,
                    self_pos,
                    self_degree,
                ),
                Self::Ellipse(..) => CollisionShape::intersect_ellipse_ellipse(
                    self,
                    self_pos,
                    self_degree,
                    other,
                    other_pos,
                    other_degree,
                ),
            },
        };

        if intersection_points.len() == 0 {
            None
        } else {
            Some(intersection_points)
        }
    }
}

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
