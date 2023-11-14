use winit::dpi::PhysicalSize;

use crate::{canvas::Drawable, Camera, Canvas, Position, Res, Vector};

const DEBUG: bool = false;

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

        let mut draw_bottom_edge = true;
        let mut draw_left_edge = true;
        let mut draw_top_edge = true;
        let mut draw_right_edge = true;

        let bottom_edge = &bottom_left_projection - &bottom_right_projection;
        let right_edge = &top_right_projection - &bottom_right_projection;
        let left_edge = &top_left_projection - &bottom_left_projection;
        let top_edge = &top_left_projection - &top_right_projection;

        let bottom_of_right_edge = if bottom_right_projection.x < 0.0 {
            //RECTANGLE IS COMPLETELY OFF SCREEN
            if top_right_projection.x < 0.0 {
                return;
            }
            if top_left_projection.x < 0.0 {
                draw_left_edge = false;
            }
            draw_bottom_edge = false;

            let delta = bottom_right_projection.x / right_edge.x;
            &bottom_right_projection + delta * &right_edge
        } else {
            bottom_right_projection.clone()
        };
        if bottom_of_right_edge.x < 0.0 || bottom_of_right_edge.y < 0.0 {
            draw_right_edge = false;
        }

        let mut bottom_of_left_edge = bottom_left_projection.clone();
        if bottom_left_projection.y < 0.0 || bottom_left_projection.x < 0.0 {
            if draw_bottom_edge {
                let y_delta = 1.0 - bottom_left_projection.y / bottom_edge.y;
                let x_delta = 1.0 - bottom_left_projection.x / bottom_edge.x;
                //TODO: MAY BE BAD TO CLAMP
                let delta = y_delta.min(x_delta).clamp(0.0, 1.0);
                // println!(
                //     "\n\ny_delta: {}\nx_delta: {}\ndelta: {}",
                //     y_delta, x_delta, delta
                // );
                let left_of_bottom_edge = &bottom_right_projection + delta * bottom_edge;
                if left_of_bottom_edge.y > 0.0 && left_of_bottom_edge.x > 0.0 {
                    self.canvas.draw_line(
                        &Position::from_vector(bottom_right_projection.clone()),
                        &Position::from_vector(left_of_bottom_edge),
                    );
                }
                draw_bottom_edge = false;
            }
            if draw_left_edge {
                let y_delta = bottom_left_projection.y / left_edge.y;
                let x_delta = bottom_left_projection.x / left_edge.x;
                //TODO: MAY BE BAD TO CLAMP
                let delta = y_delta.max(x_delta).clamp(0.0, 1.0);
                bottom_of_left_edge = &bottom_left_projection + delta * &left_edge;
                if bottom_of_left_edge.x < 0.0 || bottom_of_left_edge.y < 0.0 {
                    draw_left_edge = false;
                }
            }
        }

        let mut right_of_top_edge = top_right_projection.clone();
        if top_right_projection.y < 0.0 || top_right_projection.x < 0.0 {
            if draw_right_edge {
                let y_delta = 1.0 - top_right_projection.y / right_edge.y;
                let x_delta = 1.0 - top_right_projection.x / right_edge.x;
                //TODO: MAY BE BAD TO CLAMP
                let delta = y_delta.max(x_delta).clamp(0.0, 1.0);
                let top_of_right_edge = &bottom_right_projection + delta * right_edge;
                if top_of_right_edge.x > 0.0 && top_of_right_edge.y > 0.0 {
                    self.canvas.draw_line(
                        &Position::from_vector(bottom_of_right_edge),
                        &Position::from_vector(top_of_right_edge),
                    );
                }
                draw_right_edge = false;
            }
            if draw_top_edge {
                let y_delta = top_right_projection.y / top_edge.y;
                let x_delta = top_right_projection.x / top_edge.x;
                //TODO: MAY BE BAD TO CLAMP
                let delta = y_delta.max(x_delta).clamp(0.0, 1.0);
                right_of_top_edge = &top_right_projection + delta * &top_edge;
                if right_of_top_edge.y < 0.0 || right_of_top_edge.x < 0.0 {
                    draw_right_edge = false;
                }
            }
        }

        if top_left_projection.y < 0.0 || top_left_projection.x < 0.0 {
            if draw_left_edge {
                let y_delta = 1.0 - top_left_projection.y / left_edge.y;
                let x_delta = 1.0 - top_left_projection.x / left_edge.x;
                //TODO: MAY BE BAD TO CLAMP
                let delta = y_delta.max(x_delta).clamp(0.0, 1.0);
                let top_of_left_edge = &bottom_left_projection + delta * left_edge;
                if top_of_left_edge.y > 0.0 && top_of_left_edge.x > 0.0 {
                    self.canvas.draw_line(
                        &Position::from_vector(bottom_of_left_edge),
                        &Position::from_vector(top_of_left_edge),
                    );
                }
                draw_left_edge = false;
            }
            if draw_top_edge {
                let y_delta = 1.0 - top_left_projection.y / top_edge.y;
                let x_delta = 1.0 - top_left_projection.x / top_edge.x;
                //TODO: MAY BE BAD TO CLAMP
                let delta = y_delta.max(x_delta).clamp(0.0, 1.0);
                let left_of_top_edge = &top_right_projection + delta * top_edge;
                if left_of_top_edge.y > 0.0 && left_of_top_edge.x > 0.0 {
                    self.canvas.draw_line(
                        &Position::from_vector(left_of_top_edge),
                        &Position::from_vector(right_of_top_edge),
                    );
                }
                draw_top_edge = false;
            }
        }

        if draw_bottom_edge {
            self.canvas.draw_line(
                &Position::from_vector(bottom_right_projection.clone()),
                &Position::from_vector(bottom_left_projection.clone()),
            );
        }
        if draw_left_edge {
            self.canvas.draw_line(
                &Position::from_vector(top_left_projection.clone()),
                &Position::from_vector(bottom_left_projection.clone()),
            );
        }
        if draw_top_edge {
            self.canvas.draw_line(
                &Position::from_vector(top_left_projection.clone()),
                &Position::from_vector(top_right_projection.clone()),
            );
        }
        if draw_right_edge {
            self.canvas.draw_line(
                &Position::from_vector(bottom_right_projection.clone()),
                &Position::from_vector(top_right_projection.clone()),
            );
        }
    }
    // pub fn draw_rect(&mut self, center: &Vector, width: u32, height: u32, angel_degree: f64) {
    //     let w = width as f64 / 2.0;
    //     let h = height as f64 / 2.0;

    //     let alpha = angel_degree % 360.0;
    //     let quarter_turns = (alpha / 90.0).floor();
    //     let angel_degree = alpha - quarter_turns * 90.0;

    //     let (w, h) = if quarter_turns % 2.0 == 1.0 {
    //         (h, w)
    //     } else {
    //         (w, h)
    //     };

    //     println!("\n\ncenter: {}\n", center);
    //     println!("w: {}\nh: {}\nangel_degree: {}\n", w, h, angel_degree);
    //     let main_diagonal = Vector::new(w, h);
    //     let off_diagonal = Vector::new(-w, h);
    //     println!(
    //         "main_diagonal: {}\noff_diagonal: {}\n",
    //         main_diagonal, off_diagonal
    //     );

    //     //Vectors to the corners
    //     let top_left = (center - &main_diagonal).rotate_degree(angel_degree);
    //     let top_right = (center - &off_diagonal).rotate_degree(angel_degree);
    //     let bottom_left = (center + &off_diagonal).rotate_degree(angel_degree);
    //     let bottom_right = (center + &main_diagonal).rotate_degree(angel_degree);

    //     //Vectors to the corners in buffer world
    //     let top_left_projection = self.camera.project(&top_left);
    //     let top_right_projection = self.camera.project(&top_right);
    //     let bottom_right_projection = self.camera.project(&bottom_right);
    //     let bottom_left_projection = self.camera.project(&bottom_left);

    //     println!(
    //         "bottom_left: {}\nbottom_left_projection: {}\nbottom_right: {}\nbottom_right_projection: {}\ntop_right: {}\ntop_right_projection: {}\ntop_left: {}\ntop_left_projection: {}\n",
    //         bottom_left, bottom_left_projection,bottom_right, bottom_right_projection,top_right, top_right_projection,top_left, top_left_projection
    //     );

    //     //Constant anker, because buffer overflow in positive direction is no problem, but in negativ direction everything and everybody dies
    //     let bottom_right = Position::from_vector(bottom_right_projection.clone());

    //     //edges
    //     let mut right = &top_right_projection - &bottom_right_projection;
    //     let mut bottom = &bottom_left_projection - &bottom_right_projection;
    //     let mut left = &top_left_projection - &bottom_left_projection;
    //     let mut top = &top_left_projection - &top_right_projection;

    //     println!(
    //         "Initial edges:\nleft: {}\nright: {}\ntop: {}\nbottom: {}\n",
    //         left, right, top, bottom
    //     );

    //     let mut resized_sides = Vec::with_capacity(4);

    //     //check neighbours of ourer anker (bottom_right) for negativ overflow and destroy it
    //     if top_right_projection.y < 0.0 {
    //         let delta = 1.0 - (top_right_projection.y / right.y).abs();
    //         right *= delta;
    //         resized_sides.push("right");
    //     }
    //     if bottom_left_projection.x < 0.0 {
    //         let delta = 1.0 - (bottom_left_projection.x / bottom.x).abs();
    //         bottom *= delta;
    //         resized_sides.push("bottom");
    //     }
    //     if bottom_left_projection.y < 0.0 {
    //         let delta = 1.0 - (bottom_left_projection.y / bottom.y).abs();
    //         bottom *= delta;
    //         resized_sides.push("bottom");
    //     }

    //     println!("Intermezzo: resized_sides: {:?}", resized_sides);
    //     println!("bottom_right_projection: {}", bottom_right_projection);

    //     let top_right_projection = &bottom_right_projection + &right;
    //     println!("top_right_projection: {}", top_right_projection);
    //     let top_right = Position::from_vector(top_right_projection.clone());

    //     let bottom_left_projection = &bottom_right_projection + &bottom;
    //     println!("bottom_left_projection: {}\n", bottom_left_projection);
    //     let bottom_left = Position::from_vector(bottom_left_projection.clone());

    //     self.canvas.draw_line(&bottom_right, &top_right);
    //     self.canvas.draw_line(&bottom_right, &bottom_left);

    //     if top_left_projection.y < 0.0 {
    //         let left_delta = 1.0 - (top_left_projection.y / left.y).abs();
    //         let top_delta = 1.0 - (top_left_projection.y / top.y).abs();
    //         left *= left_delta;
    //         top *= top_delta;
    //         resized_sides.push("left");
    //         println!("left_delta: {}", left_delta);
    //         println!("resided: left: {}, top: {}", left, top);
    //     }
    //     if !resized_sides.contains(&"left") || !resized_sides.contains(&"right") {
    //         let top_left_projection = &bottom_right_projection + &right + &top;
    //         println!("top_left_projection: {}", top_left_projection);
    //         let top_left = Position::from_vector(top_left_projection);
    //         self.canvas.draw_line(&top_right, &top_left);
    //     }

    //     if top_left_projection.x < 0.0 {
    //         let delta = 1.0 - (top_left_projection.x / top.x).abs();
    //         top *= delta;
    //         resized_sides.push("top");
    //         println!("resized: top: {}", top);
    //     }
    //     if !resized_sides.contains(&"top") || !resized_sides.contains(&"bottom") {
    //         let top_left_projection = &bottom_right_projection + &bottom + &left;
    //         println!(
    //             "top_left_projection: {} = {} + {} + {}",
    //             top_left_projection, bottom_right_projection, bottom, left
    //         );
    //         let top_left = Position::from_vector(top_left_projection);
    //         self.canvas.draw_line(&bottom_left, &top_left);
    //     }

    //     println!("\nRecap:");
    //     println!("bottom_right: {}\ntop_right: {}", bottom_right, top_right);
    //     println!("bottom_left: {}\ntop_left: {}", bottom_left, top_left);

    //     // self.draw_line(&top_right, &top_left);
    //     // self.draw_line(&top_right, &bottom_right);
    //     // self.draw_line(&bottom_right, &bottom_left);
    //     // self.draw_line(&bottom_left, &top_left);
    //     // let origin = self.camera.project(&origin);
    //     // self.canvas.draw_rect(&origin, width, height)
    // }

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
