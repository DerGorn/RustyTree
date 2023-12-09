use crate::{
    camera::Camera, canvas::Canvas, canvas::Drawable, color::Color, math_2d::Vector,
    position::Position, PhysicalSize, Res,
};

impl Vector {
    /// Clamps a point into positive space while following edge
    ///
    /// returns false if self cannot be clamped along the edge
    fn clamp_point_along_edge(&mut self, edge: &Vector, is_point_edge_origin: bool) -> bool {
        // let originial = self.clone();
        let mut delta = {
            let y_delta = self.y / edge.y;
            let x_delta = self.x / edge.x;
            if !is_point_edge_origin && self.x > 0.0 && edge.x > 0.0 && self.y < 0.0 && edge.y < 0.0
            {
                y_delta.min(x_delta)
            } else if x_delta > 1.0 || x_delta < -1.0 {
                y_delta
            } else if y_delta > 1.0 || y_delta < -1.0 {
                x_delta
            } else {
                y_delta.max(x_delta)
            }
            .abs()
        };
        if delta < 0.0 || delta > 1.0 {
            // println!("False because of delta:");
            // println!(
            //     "self: {}\nedge: {}\nis_point_edge_origin: {}\ndelta: {}",
            //     self, edge, is_point_edge_origin, delta
            // );
            return false;
        }
        if !is_point_edge_origin {
            delta *= -1.0;
        }
        *self += delta * edge;
        *self = self.round();
        if self.y >= 0.0 && self.x >= 0.0 {
            // println!(
            //     "original_self: {}\nself: {}\nedge: {}\nis_point_edge_origin: {}\ndelta: {}",
            //     originial, self, edge, is_point_edge_origin, delta
            // );
            true
        } else {
            // println!("False becaus of self out of bounds:");
            // println!(
            //     "original_self: {}\nself: {}\nedge: {}\nis_point_edge_origin: {}\ndelta: {}",
            //     originial, self, edge, is_point_edge_origin, delta
            // );
            false
        }
    }
}

pub struct Renderer {
    pub camera: Camera,
    pub canvas: Canvas,
    clear_color: Color,
}
impl Renderer {
    pub fn new(camera: Camera, canvas: Canvas) -> Self {
        Renderer {
            camera,
            canvas,
            clear_color: Color::from_str("black"),
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>, new_camera: Option<Camera>) {
        match new_camera {
            Some(cam) => self.camera = cam,
            _ => {}
        }
        self.canvas.resize(new_size)
    }

    pub fn set_clear_color(&mut self, clear_color: Color) {
        self.clear_color = clear_color
    }
}

impl Renderer {
    pub fn render(&self) -> Res<()> {
        self.canvas.render()
    }

    pub fn clear(&mut self) {
        self.canvas.clear(self.clear_color.to_slice())
    }

    pub fn draw_line(&mut self, start: &Vector, end: &Vector) {
        let mut start_projection = self.camera.project(&start);
        let mut end_projection = self.camera.project(&end);
        let edge = &end_projection - &start_projection;
        if start_projection.x < 0.0 || start_projection.y < 0.0 {
            if !start_projection.clamp_point_along_edge(&edge, true) {
                return;
            }
        }
        if end_projection.x < 0.0 || end_projection.y < 0.0 {
            if !end_projection.clamp_point_along_edge(&edge, false) {
                return;
            }
        }
        self.canvas
            .draw_line(&start_projection.into(), &end_projection.into());
    }

    pub fn fill_line(&mut self, start: &Vector, end: &Vector) {
        let draw_color = self.canvas.get_draw_color();
        self.set_draw_color(self.canvas.get_fill_color());
        self.draw_line(start, end);
        self.set_draw_color(draw_color);
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

            let x = x as f64;
            let y = -height;
            if first {
                bottom = center + Vector::new(x, 0.0).rotate_degree(angel_degree);
                top = bottom.clone();
                first = false;
            }

            let target = center + Vector::new(x, y).rotate_degree(angel_degree);

            self.draw_line(&bottom, &target);

            bottom = target;

            let y = height;
            let target = center + Vector::new(x, y).rotate_degree(angel_degree);

            self.draw_line(&top, &target);

            top = target;
        }
    }

    ///TODO: FUCKING MYSTICAL STUFF. I CANT DO ANYTHING HERE WITHOUT IT DETONATING. WRITE THIS SHIT BETTER!!!!!
    pub fn fill_ellipse(&mut self, center: &Vector, a: u32, b: u32, angel_degree: f64) {
        let mut a = a as f64;
        let mut b = b as f64;
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

        let point_on_ellipse = center + Vector::new(0.0, b).rotate_degree(alpha);
        let distance = point_on_ellipse.distance(&f1) + point_on_ellipse.distance(&f2);

        let is_inside = |vector: &Vector| vector.distance(&f1) + vector.distance(&f2) < distance;

        let dimensions = Vector::new(a + 1.0, a + 1.0);
        let min = center - &dimensions;
        let max = center + &dimensions;

        let color = self.canvas.get_fill_color();

        for x in min.x as i32..=max.x as i32 {
            for y in min.y as i32..=max.y as i32 {
                let vector = Vector::new(x as f64, y as f64);
                if is_inside(&vector) {
                    self.set_pixel(&vector, &color)
                }
            }
        }
    }

    ///TODO: CORNERS ARE DRAWN TWICE => TOO BRIGHT
    pub fn draw_rect(&mut self, center: &Vector, width: u32, height: u32, angel_degree: f64) {
        let w = width as f64 / 2.0;
        let h = height as f64 / 2.0;

        let alpha = angel_degree % 360.0;
        let quarter_turns = (alpha / 90.0).floor();
        let alpha = alpha - quarter_turns * 90.0;

        let (w, h) = if quarter_turns % 2.0 == 1.0 {
            (h, w)
        } else {
            (w, h)
        };

        let main_diagonal = Vector::new(w, h);
        let off_diagonal = Vector::new(-w, h);

        //Basic Corners
        let bottom_right = (center + &main_diagonal).rotate_degree(alpha);
        let top_left = (center - &main_diagonal).rotate_degree(alpha);
        let bottom_left = (center + &off_diagonal).rotate_degree(alpha);
        let top_right = (center - &off_diagonal).rotate_degree(alpha);

        //Projected Corners into Bufferspace
        let bottom_right_projection = self.camera.project(&bottom_right);
        let bottom_left_projection = self.camera.project(&bottom_left);
        let top_left_projection = self.camera.project(&top_left);
        let top_right_projection = self.camera.project(&top_right);

        //RECTANGLE IS COMPLETELY OFF SCREEN
        if bottom_right_projection.y < 0.0 {
            return;
        }

        let mut draw_bottom_edge = true;
        let mut draw_left_edge = true;
        let mut draw_top_edge = true;
        let mut draw_right_edge = true;

        let bottom_edge = &bottom_left_projection - &bottom_right_projection;
        let right_edge = &top_right_projection - &bottom_right_projection;
        let left_edge = &top_left_projection - &bottom_left_projection;
        let top_edge = &top_left_projection - &top_right_projection;

        let right_of_bottom_edge = bottom_right_projection.clone();
        let mut bottom_of_right_edge = bottom_right_projection.clone();
        if bottom_right_projection.x < 0.0 {
            //RECTANGLE IS COMPLETELY OFF SCREEN
            if top_right_projection.x < 0.0 {
                return;
            }
            if top_left_projection.x < 0.0 {
                draw_left_edge = false;
            }
            draw_bottom_edge = false;

            if draw_right_edge {
                draw_right_edge = bottom_of_right_edge.clamp_point_along_edge(&right_edge, true);
            }
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
            self.canvas
                .draw_line(&right_of_bottom_edge.into(), &left_of_bottom_edge.into());
        }
        if draw_right_edge {
            self.canvas
                .draw_line(&bottom_of_right_edge.into(), &top_of_right_edge.into());
        }
        if draw_left_edge {
            self.canvas
                .draw_line(&top_of_left_edge.into(), &bottom_of_left_edge.into());
        }
        if draw_top_edge {
            self.canvas
                .draw_line(&left_of_top_edge.into(), &right_of_top_edge.into());
        }
    }

    pub fn fill_rect(&mut self, center: &Vector, width: u32, height: u32, angel_degree: f64) {
        let w = width as f64 / 2.0;
        let h = height as f64 / 2.0;

        let alpha = angel_degree % 360.0;
        let quarter_turns = (alpha / 90.0).floor();
        let alpha = alpha - quarter_turns * 90.0;

        let (w, h) = if quarter_turns % 2.0 == 1.0 {
            (h, w)
        } else {
            (w, h)
        };

        let main_diagonal = Vector::new(w, h);
        let off_diagonal = Vector::new(-w, h);

        //Basic Corners
        let bottom_right = (center + &main_diagonal).rotate_degree(alpha);
        let top_left = (center - &main_diagonal).rotate_degree(alpha);
        let bottom_left = (center + &off_diagonal).rotate_degree(alpha);
        let top_right = (center - &off_diagonal).rotate_degree(alpha);

        //Projected Corners into Bufferspace
        let bottom_right_projection = self.camera.project(&bottom_right);
        let bottom_left_projection = self.camera.project(&bottom_left);
        let top_left_projection = self.camera.project(&top_left);
        let top_right_projection = self.camera.project(&top_right);

        //RECTANGLE IS COMPLETELY OFF SCREEN
        if bottom_right_projection.y < 0.0
            || (bottom_right_projection.x < 0.0 && top_right_projection.x < 0.0)
        {
            return;
        }

        let left_of_top = &top_left_projection - &bottom_left_projection;
        let mut left_top_slope = left_of_top.y / left_of_top.x;
        if left_top_slope.is_infinite() {
            left_top_slope = 0.0;
        }
        let right_of_top = &top_right_projection - &top_left_projection;
        let mut right_top_slope = right_of_top.y / right_of_top.x;
        if right_top_slope.is_infinite() {
            right_top_slope = 0.0;
        }

        let x_min = bottom_left_projection.x.max(0.0) as u32;
        let x_max = top_right_projection.x.min(self.get_width() as f64 - 1.0) as u32;

        let global_y_min = top_left_projection.y.max(0.0);
        let global_y_max = bottom_right_projection
            .y
            .min(self.get_height() as f64 - 1.0);

        let y_min = |x: f64| -> u32 {
            let y_min = if x < top_left_projection.x {
                let x = x - x_min as f64;
                bottom_left_projection.y + x * left_top_slope
            } else if x > top_left_projection.x {
                let x = x - top_left_projection.x as f64;
                top_left_projection.y + x * right_top_slope
            } else {
                top_left_projection.y
            }
            .max(global_y_min) as u32;
            y_min
        };

        let y_max = |x: f64| -> u32 {
            if x < bottom_right_projection.x {
                let x = x - x_min as f64;
                bottom_left_projection.y + x * right_top_slope
            } else if x > bottom_right_projection.x {
                let x = x - bottom_right_projection.x;
                bottom_right_projection.y + x * left_top_slope
            } else {
                bottom_right_projection.y
            }
            .min(global_y_max) as u32
        };

        let color = self.canvas.get_fill_color();

        for x in x_min..=x_max {
            for y in y_min(x as f64)..=y_max(x as f64) {
                self.canvas.set_pixel(&Position::new(x, y), &color);
            }
        }
    }

    pub fn fill_pixel(&mut self, position: &Vector) {
        self.set_pixel(position, &self.canvas.get_fill_color())
    }

    pub fn draw_pixel(&mut self, position: &Vector) {
        self.set_pixel(position, &self.canvas.get_draw_color())
    }

    fn set_pixel(&mut self, position: &Vector, color: &Color) {
        let position = self.camera.clamped_projection_to_position(&position);
        if position.x < self.get_width() && position.y < self.get_height() {
            self.canvas.set_pixel(&position, color)
        }
    }

    pub fn set_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color.to_rgba())
    }

    pub fn set_fill_color(&mut self, color: Color) {
        self.canvas.set_fill_color(color.to_rgba())
    }

    pub fn get_height(&self) -> u32 {
        self.canvas.get_height()
    }

    pub fn get_width(&self) -> u32 {
        self.canvas.get_width()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_offscreen_line() {
        let size = PhysicalSize::new(4, 4);
        let mut canvas = Renderer::new(
            Camera::new(Vector::zero()),
            Canvas::new_with_simplebuffer(size),
        );
        canvas.clear();

        let start = Vector::new(0.0, -2.0);
        let end = Vector::new(4.0, 2.0);

        canvas.set_draw_color(Color::from_str("red"));
        canvas.draw_line(&start, &end);

        let p1 = Position::new(1, 0);
        let index1 = ((size.width * p1.y + p1.x) * 4) as usize;
        let p2 = Position::new(0, 0);
        let index2 = ((size.width * p2.y + p2.x) * 4) as usize;

        let buffer = canvas.canvas.as_slice();
        assert_ne!(255, buffer[index1]);
        assert_ne!(255, buffer[index2]);
    }
}
