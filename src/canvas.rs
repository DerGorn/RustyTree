use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, window::Window};

use crate::Res;

use crate::color::Color;

use crate::buffer::Buffer;
use crate::buffer::SimpleBuffer;

use crate::Position;

pub trait Drawable<T> {
    fn set_draw_color(&mut self, color: Color);

    fn set_fill_color(&mut self, color: Color);

    fn get_width(&self) -> u32;

    fn get_height(&self) -> u32;

    fn render(&self) -> Res<()>;

    fn clear(&mut self, clear_value: u8);

    fn set_pixel(&mut self, position: &T, color: Color);

    fn draw_line(&mut self, start: &T, end: &T);

    fn draw_ellipse(&mut self, center: &T, a: u32, b: u32);

    fn fill_ellipse(&mut self, center: &T, a: u32, b: u32);

    fn draw_rect(&mut self, origin: &T, width: u32, height: u32);

    fn fill_rect(&mut self, origin: &T, width: u32, height: u32);
}

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

    pub fn as_slice(&mut self) -> &mut [u8] {
        self.buffer.buffer()
    }

    fn fill_pixel(buffer: &mut [u8], x: u32, y: u32, buffer_width: u32, color: [u8; 4]) {
        let index = ((buffer_width * y + x) * 4) as usize;
        let color = if color[3] != 255 {
            Color::rgba_from_slice(&color)
                .blend(&Color::rgba_from_slice(
                    buffer.get(index..=index + 3).unwrap().try_into().unwrap(),
                ))
                .to_slice()
        } else {
            color
        };
        buffer[index] = color[0];
        buffer[index + 1] = color[1];
        buffer[index + 2] = color[2];
        buffer[index + 3] = color[3];
    }

    pub fn resize(&mut self, buffer_size: PhysicalSize<u32>) {
        self.size = buffer_size;
        self.buffer.resize(buffer_size)
    }

    pub fn get_draw_color(&self) -> Color {
        self.draw_color.clone()
    }
    pub fn get_fill_color(&self) -> Color {
        self.fill_color.clone()
    }
}
impl Drawable<Position> for Canvas {
    fn set_draw_color(&mut self, color: Color) {
        self.draw_color = color
    }
    fn set_fill_color(&mut self, color: Color) {
        self.fill_color = color
    }

    fn render(&self) -> Res<()> {
        self.buffer.render()
    }

    fn set_pixel(&mut self, position: &Position, color: Color) {
        Self::fill_pixel(
            self.buffer.buffer(),
            position.x,
            position.y,
            self.size.width,
            color.to_slice(),
        )
    }

    fn get_width(&self) -> u32 {
        self.size.width
    }

    fn get_height(&self) -> u32 {
        self.size.height
    }

    fn clear(&mut self, clear_value: u8) {
        self.buffer.clear(clear_value)
    }

    fn draw_line(&mut self, start: &Position, end: &Position) {
        let buffer = self.buffer.buffer();

        let color = self.draw_color.to_slice();
        let rise = end.y as f64 - start.y as f64;
        let mut run = end.x as f64 - start.x as f64;
        if run == 0.0 {
            run = 1.0;
        }

        let slope = rise / run;
        let slope_direction = slope.signum();
        let slope = slope.abs();
        let mut current_slope = slope;

        let mut last_y = if run > 0.0 { start.y } else { end.y };

        let x_range = if run < 0.0 {
            (start.x as f64 + run) as u32..start.x
        } else {
            start.x..(start.x as f64 + run) as u32
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
                if last_y >= self.size.height {
                    continue;
                }
                Self::fill_pixel(buffer, x, last_y, self.size.width, color);
                continue;
            }
            let y_range = 0..height as i32;
            for y in y_range {
                let y = (last_y as i32 + slope_direction as i32 * y) as u32;
                if y >= self.size.height {
                    continue;
                }
                Self::fill_pixel(buffer, x, y, self.size.width, color);
            }
            last_y = (last_y as f64 + (slope_direction * height as f64)) as u32;
        }
    }

    fn draw_ellipse(&mut self, center: &Position, a: u32, b: u32) {
        let a = a as i64;

        let mut top_y = center.y as i64;
        let mut bottom_y = center.y as i64;

        for x in -a..=a {
            let height =
                ((1.0 - x.pow(2) as f64 / a.pow(2) as f64) * b.pow(2) as f64).sqrt() as i64;

            let x = center.x as f64 + x as f64;
            if x < 0.0 {
                bottom_y = center.y as i64 - height;
                top_y = center.y as i64 + height;
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
            self.draw_line(&Position::new(x, bottom_y as u32), &Position::new(x, y));
            bottom_y = y as i64;
            let y = center.y + height as u32;
            if y >= self.size.height {
                continue;
            }
            self.draw_line(&Position::new(x, top_y as u32), &Position::new(x, y));
            top_y = y as i64;
        }
    }

    fn fill_ellipse(&mut self, center: &Position, a: u32, b: u32) {
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

    fn draw_rect(&mut self, origin: &Position, width: u32, height: u32) {
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

    fn fill_rect(&mut self, origin: &Position, width: u32, height: u32) {
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
