use std::error::Error;

mod vector;
use buffer::SimpleBuffer;
pub use vector::Vector;

mod color;
pub use color::Color;

mod buffer;
pub use buffer::Buffer;

use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, window::Window};

type Res<T, E = Box<dyn Error>> = Result<T, E>;

pub struct Canvas {
    size: PhysicalSize<u32>,
    buffer: Box<dyn Buffer>,
    draw_color: Color,
    fill_color: Color,
}
impl Canvas {
    pub fn new_with_pixels(buffer_size: PhysicalSize<u32>, window: &Window) -> Res<Self> {
        let buffer = Pixels::new(
            buffer_size.width,
            buffer_size.height,
            SurfaceTexture::new(buffer_size.width, buffer_size.height, &window),
        )?;
        let buffer = Box::new(buffer);
        Ok(Self::new_from_buffer(buffer_size, buffer))
    }

    pub fn new_with_simplebuffer(buffer_size: PhysicalSize<u32>) -> Self {
        let buffer = Box::new(SimpleBuffer::new(buffer_size));
        Self::new_from_buffer(buffer_size, buffer)
    }

    pub fn new_from_buffer(buffer_size: PhysicalSize<u32>, buffer: Box<dyn Buffer>) -> Self {
        Self {
            size: buffer_size,
            buffer,
            draw_color: Color::from_str("black"),
            fill_color: Color::from_str("white"),
        }
    }

    pub fn set_draw_color(&mut self, color: Color) {
        self.draw_color = color
    }
    pub fn set_fill_color(&mut self, color: Color) {
        self.fill_color = color
    }

    pub fn as_slice(&mut self) -> &mut [u8] {
        self.buffer.buffer()
    }

    pub fn resize(&mut self, buffer_size: PhysicalSize<u32>) {
        self.buffer.resize(buffer_size)
    }

    pub fn render(&self) -> Res<()> {
        self.buffer.render()
    }

    fn fill_pixel(buffer: &mut [u8], x: u32, y: u32, width: u32, color: [u8; 4]) {
        let index = ((width * y + x) * 4) as usize;
        buffer[index] = color[0];
        buffer[index + 1] = color[1];
        buffer[index + 2] = color[2];
        buffer[index + 3] = color[3];
    }

    pub fn get_width(&self) -> u32 {
        self.size.width
    }

    pub fn get_height(&self) -> u32 {
        self.size.height
    }

    pub fn clear(&mut self, clear_value: u8) {
        self.buffer.clear(clear_value)
    }

    pub fn draw_line(&mut self, start: Vector, end: Vector) {
        let buffer = self.buffer.buffer();

        let color = self.draw_color.to_slice();
        let rise = end.y as f64 - start.y as f64;
        let mut run = end.x as f64 - start.x as f64;
        if run == 0.0 {
            run = 1.0;
        }

        let y_direction = rise.signum();
        let x_direction = run.signum();
        let y_modulator = x_direction * y_direction;

        let slope = (rise / run).abs();
        let mut current_slope = slope;

        let mut last_y = if x_direction > 0.0 { start.y } else { end.y };

        let x_range = if x_direction < 0.0 {
            (start.x as i32 + (run as i32)) as u32..start.x
        } else {
            start.x..start.x + run as u32
        };
        for x in x_range {
            if x >= self.size.width {
                break;
            }
            let height = current_slope.floor() as u32;
            if current_slope.floor() > slope.floor() {
                current_slope -= 1.0;
            }
            current_slope += slope.fract();
            if height == 0 {
                Self::fill_pixel(buffer, x, last_y, self.size.width, color);
                continue;
            }
            let y_range = 0..height as i32;
            for y in y_range {
                let y = (last_y as i32 + y_modulator as i32 * y) as u32;
                if y >= self.size.height {
                    break;
                }
                Self::fill_pixel(buffer, x, y, self.size.width, color);
            }
            last_y = (last_y as f64 + (y_modulator * height as f64)) as u32;
        }
    }

    pub fn draw_ellipse(&mut self, center: Vector, a: u32, b: u32) {
        let a = a as i64;

        let mut top_y = center.y as i64;
        let mut bottom_y = center.y as i64;

        for x in -a..=a {
            let height =
                ((1.0 - x.pow(2) as f64 / a.pow(2) as f64) * b.pow(2) as f64).sqrt() as i64;

            let x = center.x as f64 + x as f64;
            if x < 0.0 {
                continue;
            }
            let x = x as u32;
            if x >= self.size.width {
                break;
            }
            let y = (center.y as i64 - height) as u32;
            if y >= self.size.height {
                continue;
            }
            self.draw_line(Vector::new(x, bottom_y as u32), Vector::new(x, y));
            bottom_y = y as i64;
            let y = center.y + height as u32;
            if y >= self.size.height {
                continue;
            }
            self.draw_line(Vector::new(x, top_y as u32), Vector::new(x, y));
            top_y = y as i64;
        }
    }

    pub fn fill_ellipse(&mut self, center: Vector, a: u32, b: u32) {
        let buffer = self.buffer.buffer();

        let a = a as i64;
        let color = self.fill_color.to_slice();

        for x in -a..=a {
            let x = x as f64;
            let height = ((1.0 - x.powi(2) / a.pow(2) as f64) * b.pow(2) as f64).sqrt() as i32;

            let x = (center.x as f64 + x) as u32;
            if x >= self.size.width {
                break;
            }
            for y in (center.y as i32 - height) as u32..(center.y as i32 + height) as u32 {
                if y >= self.size.height {
                    break;
                }
                Self::fill_pixel(buffer, x, y, self.size.width, color);
            }
        }
    }

    pub fn draw_rect(&mut self, origin: Vector, width: u32, height: u32) {
        let buffer = self.buffer.buffer();

        let color = self.draw_color.to_slice();

        let x = origin.x;
        if x >= self.size.width {
            return ();
        }
        for y in origin.y..origin.y + height {
            if y >= self.size.height {
                break;
            }
            Self::fill_pixel(buffer, x, y, self.size.width, color);
        }
        let x = origin.x + width;
        if x < self.size.width {
            for y in origin.y..origin.y + height {
                if y >= self.size.height {
                    break;
                }
                Self::fill_pixel(buffer, x, y, self.size.width, color);
            }
        }
        for x in origin.x..origin.x + width {
            if x >= self.size.width {
                break;
            }
            let y = origin.y;
            if y >= self.size.height {
                break;
            }
            Self::fill_pixel(buffer, x, y, self.size.width, color);
            let y = origin.y + height;
            if y >= self.size.height {
                break;
            }
            Self::fill_pixel(buffer, x, y, self.size.width, color);
        }
    }

    pub fn fill_rect(&mut self, origin: Vector, width: u32, height: u32) {
        let buffer = self.buffer.buffer();

        let color = self.fill_color.to_slice();

        for x in origin.x..origin.x + width {
            if x >= self.size.width {
                break;
            }
            for y in origin.y..origin.y + height {
                if y >= self.size.height {
                    break;
                }
                Self::fill_pixel(buffer, x, y, self.size.width, color);
            }
        }
    }
}
