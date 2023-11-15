use winit::dpi::PhysicalSize;

use crate::{canvas::Drawable, Camera, Canvas, Position, Res, Vector};

const DEBUG: bool = false;

impl Vector {
    /// Clamps a point into positive space while following edge
    ///
    /// returns false if self cannot be clamped along the edge
    fn clamp_point_along_edge(&mut self, edge: &Vector, is_point_edge_origin: bool) -> bool {
        println!(
            "self: {}\nedge: {}\nis_point_origin: {}",
            self, edge, is_point_edge_origin
        );
        let mut delta = {
            let y_delta = self.y / edge.y;
            let x_delta = self.x / edge.x;

            y_delta.max(x_delta)
        };
        println!("delta: {}", delta);
        if delta < 0.0 || delta > 1.0 {
            return false;
        }
        if !is_point_edge_origin {
            delta *= -1.0;
        }
        *self += delta * edge;
        *self = self.round();
        if self.y >= 0.0 && self.x >= 0.0 {
            true
        } else {
            false
        }
    }
}

pub struct Renderer {
    pub camera: Camera,
    pub canvas: Canvas,
}
impl Renderer {
    pub fn new(camera: Camera, canvas: Canvas) -> Self {
        Renderer { camera, canvas }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>, new_camera: Option<Camera>) {
        match new_camera {
            Some(cam) => self.camera = cam,
            _ => {}
        }
        self.canvas.resize(new_size)
    }
}

impl Renderer {
    pub fn render(&self) -> Res<()> {
        self.canvas.render()
    }

    pub fn clear(&mut self, clear_value: u8) {
        self.canvas.clear(clear_value)
    }

    pub fn draw_ellipse(&mut self, center: &Vector, a: u32, b: u32, angel_degree: f64) {
        let alpha = angel_degree % 360.0;
        let quarter_turns = (alpha / 90.0).floor();
        let angel_degree = alpha - quarter_turns * 90.0;

        let (a, b) = if quarter_turns % 2.0 == 1.0 {
            (b, a)
        } else {
            (a, b)
        };

        let a = a as i64;

        let mut first = true;
        let mut top = Vector::zero();
        let mut bottom = Vector::zero();

        for x in -a..=a {
            let height = ((1.0 - x.pow(2) as f64 / a.pow(2) as f64) * b.pow(2) as f64).sqrt();

            let x = center.x + x as f64;
            let y = center.y - height;
            if first {
                bottom = Vector::new(x, center.y).rotate_degree(angel_degree);
                top = Vector::new(x, center.y).rotate_degree(angel_degree);
                first = false;
            }

            let target = Vector::new(x, y).rotate_degree(angel_degree);

            self.draw_line(&bottom, &target);

            bottom = target;

            let y = center.y + height;
            let target = Vector::new(x, y).rotate_degree(angel_degree);

            self.draw_line(&top, &target);

            top = target;
        }
    }

    pub fn fill_ellipse(&mut self, center: &Vector, a: u32, b: u32, angel_degree: f64) {
        let mut a = a as f64 / 2.0;
        let mut b = b as f64 / 2.0;
        let mut alpha = angel_degree;

        if a < b {
            (a, b) = (b, a);
            alpha -= 90.0;
        }

        let focal_distance = (a.powi(2) - b.powi(2)).sqrt();
        let focal_offset = if alpha != angel_degree {
            Vector::new(0.0, focal_distance)
        } else {
            Vector::new(focal_distance, 0.0)
        }
        .rotate_degree(angel_degree);
        let f1 = center + &focal_offset;
        let f2 = center - &focal_offset;

        let point_on_ellipse = (center + Vector::new(0.0, b)).rotate_degree(alpha);
        let distance = point_on_ellipse.distance(&f1) + point_on_ellipse.distance(&f2);

        let is_inside = |vector: &Vector| vector.distance(&f1) + vector.distance(&f2) < distance;

        let dimensions = Vector::new(a + 1.0, a + 1.0);
        let min = center - &dimensions;
        let max = center + &dimensions;

        let color = self.canvas.get_fill_color();

        for x in min.x as i64..max.x as i64 {
            for y in min.y as i64..max.y as i64 {
                let vector = Vector::new(x as f64, y as f64);
                if is_inside(&vector) {
                    self.set_pixel(&vector, color.clone())
                }
            }
        }
    }

    pub fn draw_line(&mut self, start: &Vector, end: &Vector) {
        let start = self.camera.clamped_projection_to_position(&start);
        let end = self.camera.clamped_projection_to_position(&end);
        self.canvas.draw_line(&start, &end)
    }

    pub fn draw_rect(&mut self, center: &Vector, width: u32, height: u32, angel_degree: f64) {
        if DEBUG {
            println!(
                "\n\n--------------New Rectangle-------------\ncenter: {}\n",
                center
            );
        }
        let w = width as f64 / 2.0;
        let h = height as f64 / 2.0;

        let alpha = angel_degree % 360.0;
        let quarter_turns = (alpha / 90.0).floor();
        let alpha = alpha - quarter_turns * 90.0;

        if DEBUG {
            println!("quarter_turns: {}\nalpha: {}\n", quarter_turns, alpha);
        }

        let (w, h) = if quarter_turns % 2.0 == 1.0 {
            (h, w)
        } else {
            (w, h)
        };

        let main_diagonal = Vector::new(w, h);
        let off_diagonal = Vector::new(-w, h);

        if DEBUG {
            println!(
                "main_diagonal: {}\noff_diagonal: {}\n",
                main_diagonal, off_diagonal
            );
        }

        //Basic Corners
        let bottom_right = (center + &main_diagonal).rotate_degree(alpha);
        let top_left = (center - &main_diagonal).rotate_degree(alpha);
        let bottom_left = (center + &off_diagonal).rotate_degree(alpha);
        let top_right = (center - &off_diagonal).rotate_degree(alpha);

        if DEBUG {
            println!(
                "bottom_right: {}\nbottom_left: {}\ntop_left: {}\ntop_right: {}\n",
                bottom_right, bottom_left, top_left, top_right
            );
        }

        //Projected Corners into Bufferspace
        let bottom_right_projection = self.camera.project(&bottom_right);
        let bottom_left_projection = self.camera.project(&bottom_left);
        let top_left_projection = self.camera.project(&top_left);
        let top_right_projection = self.camera.project(&top_right);

        //RECTANGLE IS COMPLETELY OFF SCREEN
        if bottom_right_projection.y < 0.0 {
            return;
        }

        if DEBUG {
            println!("Projected into Bufferspace:");
            println!(
                "bottom_right: {}\nbottom_left: {}\ntop_left: {}\ntop_right: {}\n",
                bottom_right_projection,
                bottom_left_projection,
                top_left_projection,
                top_right_projection
            );
        }

        let mut draw_bottom_edge = false;
        let mut draw_left_edge = true;
        let mut draw_top_edge = false;
        let mut draw_right_edge = false;

        let bottom_edge = &bottom_left_projection - &bottom_right_projection;
        let right_edge = &top_right_projection - &bottom_right_projection;
        let left_edge = &top_left_projection - &bottom_left_projection;
        let top_edge = &top_left_projection - &top_right_projection;

        let right_of_bottom_edge = bottom_right_projection.clone();
        let bottom_of_right_edge = if bottom_right_projection.x < 0.0 {
            //RECTANGLE IS COMPLETELY OFF SCREEN
            if top_right_projection.x < 0.0 {
                return;
            }
            // if top_left_projection.x < 0.0 {
            //     draw_left_edge = false;
            // }
            draw_bottom_edge = false;

            let delta = bottom_right_projection.x / right_edge.x;
            &bottom_right_projection + delta * &right_edge
        } else {
            bottom_right_projection.clone()
        };
        if bottom_of_right_edge.x < 0.0 || bottom_of_right_edge.y < 0.0 {
            draw_right_edge = false;
        }

        let mut left_of_bottom_edge = bottom_left_projection.clone();
        let mut bottom_of_left_edge = bottom_left_projection.clone();
        if bottom_left_projection.y < 0.0 || bottom_left_projection.x < 0.0 {
            if draw_bottom_edge {
                draw_bottom_edge = left_of_bottom_edge.clamp_point_along_edge(&bottom_edge, false);
            }
            if draw_left_edge {
                draw_left_edge = bottom_of_left_edge.clamp_point_along_edge(&left_edge, true);
            }
        }

        let mut top_of_right_edge = top_right_projection.clone();
        let mut right_of_top_edge = top_right_projection.clone();
        if top_right_projection.y < 0.0 || top_right_projection.x < 0.0 {
            if draw_right_edge {
                draw_right_edge = top_of_right_edge.clamp_point_along_edge(&right_edge, false);
            }
            if draw_top_edge {
                draw_top_edge = right_of_top_edge.clamp_point_along_edge(&top_edge, true);
            }
        }

        let mut top_of_left_edge = top_left_projection.clone();
        let mut left_of_top_edge = top_left_projection.clone();
        if top_left_projection.y < 0.0 || top_left_projection.x < 0.0 {
            if draw_left_edge {
                draw_left_edge = top_of_left_edge.clamp_point_along_edge(&left_edge, false);
            }
            if draw_top_edge {
                draw_top_edge = left_of_top_edge.clamp_point_along_edge(&top_edge, false);
            }
        }

        if draw_bottom_edge {
            self.canvas.draw_line(
                &Position::from_vector(right_of_bottom_edge),
                &Position::from_vector(left_of_bottom_edge),
            );
        }
        if draw_right_edge {
            self.canvas.draw_line(
                &Position::from_vector(bottom_of_right_edge),
                &Position::from_vector(top_of_right_edge),
            );
        }
        if draw_left_edge {
            self.canvas.draw_line(
                &Position::from_vector(top_of_left_edge),
                &Position::from_vector(bottom_of_left_edge),
            );
        } else {
            println!(
                "\n\ntop_of_left_edge: {}\nbottom_of_left_edge: {}",
                top_of_left_edge, bottom_of_left_edge
            );
        }
        if draw_top_edge {
            self.canvas.draw_line(
                &Position::from_vector(left_of_top_edge),
                &Position::from_vector(right_of_top_edge),
            );
        }
    }

    pub fn fill_rect(&mut self, origin: &Vector, width: u32, height: u32) {
        let origin = self.camera.clamped_projection_to_position(&origin);
        self.canvas.fill_rect(&origin, width, height)
    }

    pub fn set_pixel(&mut self, position: &Vector, color: crate::Color) {
        let position = self.camera.clamped_projection_to_position(&position);
        if position.x < self.get_width() && position.y < self.get_height() {
            self.canvas.set_pixel(&position, color)
        }
    }

    pub fn set_draw_color(&mut self, color: crate::Color) {
        self.canvas.set_draw_color(color.to_rgba())
    }

    pub fn set_fill_color(&mut self, color: crate::Color) {
        self.canvas.set_fill_color(color.to_rgba())
    }

    pub fn get_height(&self) -> u32 {
        self.canvas.get_height()
    }

    pub fn get_width(&self) -> u32 {
        self.canvas.get_width()
    }
}
