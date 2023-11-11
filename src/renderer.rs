use winit::dpi::PhysicalSize;

use crate::{canvas::Drawable, Camera, Canvas, Res, Vector};

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

        let point_on_ellipse = (center + Vector::new(0.0, b)).rotate_degree(alpha);
        let distance = point_on_ellipse.distance(&f1) + point_on_ellipse.distance(&f2);

        let inside = |vector: &Vector| vector.distance(&f1) + vector.distance(&f2) < distance;

        let dimensions = Vector::new(a + 1.0, a + 1.0);
        let min = center - &dimensions;
        let max = center + &dimensions;

        let color = self.canvas.get_fill_color();

        for x in min.x as i64..max.x as i64 {
            for y in min.y as i64..max.y as i64 {
                let vector = Vector::new(x as f64, y as f64);
                if inside(&vector) {
                    self.set_pixel(&vector, color.clone())
                }
            }
        }
    }

    pub fn fill_ellipse_old(&mut self, center: &Vector, a: u32, b: u32, angel_degree: f64) {
        let a = a as i64;

        let draw_color = self.canvas.get_draw_color();
        self.set_draw_color(self.canvas.get_fill_color());
        for x in -a..=a {
            let height = ((1.0 - x.pow(2) as f64 / a.pow(2) as f64) * b.pow(2) as f64).sqrt();

            let x = center.x + x as f64;
            let bottom_y = center.y - height;
            let top_y = center.y + height;
            self.draw_line(
                &Vector::new(x, bottom_y).rotate_degree(angel_degree),
                &Vector::new(x, top_y).rotate_degree(angel_degree),
            )
        }
        self.set_draw_color(draw_color);
    }

    pub fn draw_line(&mut self, start: &Vector, end: &Vector) {
        let start = self.camera.project(&start);
        let end = self.camera.project(&end);
        self.canvas.draw_line(&start, &end)
    }

    pub fn draw_rect(&mut self, origin: &Vector, width: u32, height: u32) {
        let origin = self.camera.project(&origin);
        self.canvas.draw_rect(&origin, width, height)
    }

    pub fn fill_rect(&mut self, origin: &Vector, width: u32, height: u32) {
        let origin = self.camera.project(&origin);
        self.canvas.fill_rect(&origin, width, height)
    }

    pub fn set_pixel(&mut self, position: &Vector, color: crate::Color) {
        let position = self.camera.project(&position);
        if position.x < self.get_width() && position.y < self.get_height() {
            self.canvas.set_pixel(&position, color)
        }
    }

    pub fn set_draw_color(&mut self, color: crate::Color) {
        self.canvas.set_draw_color(color)
    }

    pub fn set_fill_color(&mut self, color: crate::Color) {
        self.canvas.set_fill_color(color)
    }

    pub fn get_height(&self) -> u32 {
        self.canvas.get_height()
    }

    pub fn get_width(&self) -> u32 {
        self.canvas.get_width()
    }
}
