use crate::PhysicalSize;
use crate::Res;

use crate::color::Color;

use crate::buffer::Buffer;
use crate::buffer::SimpleBuffer;

use crate::position::Position;

pub trait Drawable<T> {
    fn set_draw_color(&mut self, color: Color);

    fn set_fill_color(&mut self, color: Color);

    fn get_width(&self) -> u32;

    fn get_height(&self) -> u32;

    fn render(&self) -> Res<()>;

    fn clear(&mut self, clear_value: u8);

    fn set_pixel(&mut self, position: &T, color: Color);

    fn draw_line(&mut self, start: &T, end: &T);
}

pub struct Canvas {
    size: PhysicalSize<u32>,
    buffer: Box<dyn Buffer>,
    draw_color: Color,
    fill_color: Color,
}
impl Canvas {
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
                    match buffer.get(index..=index + 3) {
                        None => panic!(
                            "Index out of bounds. Buffer is only {} bytes big, but index is {}",
                            buffer.len(),
                            index
                        ),
                        Some(pixel) => pixel,
                    }
                    .try_into()
                    .unwrap(),
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
        let slope_direction = slope.signum() as i32;
        let slope = slope.abs();
        let slope_floor = slope.floor() as i32;
        let slope_fract = slope.fract();
        let special_slope = slope == 2.0;
        let half_slope = (slope / 2.0).abs() as u32;
        let mut current_slope = slope;

        let mut last_y = if run > 0.0 { start.y } else { end.y };

        let (x_start, x_end) = if run < 0.0 {
            ((start.x as f64 + run) as u32, start.x)
        } else {
            (start.x, (start.x as f64 + run) as u32)
        };
        let x_range = x_start.max(0)..x_end.min(self.size.width);

        for x in x_range {
            let mut height = current_slope.floor() as i32;
            if x == x_start && special_slope {
                height -= half_slope as i32;
            }

            if height > slope_floor {
                current_slope -= 1.0;
            }
            current_slope += slope_fract;
            if height == 0 {
                if last_y < self.size.height {
                    Self::fill_pixel(buffer, x, last_y, self.size.width, color);
                }
                continue;
            }

            let y_end = (last_y as i32 + slope_direction * height) as u32;
            let y_range = if slope_direction == 1 {
                last_y.max(0)..y_end.min(self.size.height)
            } else {
                y_end.max(0)..last_y.min(self.size.height)
            };

            for y in y_range {
                Self::fill_pixel(buffer, x, y, self.size.width, color);
            }
            last_y = y_end;
        }
        if special_slope && last_y >= half_slope {
            last_y -= half_slope;
        }
        if x_end < self.size.width && last_y < self.size.height {
            Self::fill_pixel(buffer, x_end, last_y, self.size.width, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    #[test]
    pub fn draw_line() -> Res<()> {
        let size = PhysicalSize::new(10, 10);
        let mut canvas = Canvas::new_with_simplebuffer(size);
        canvas.clear(100);

        let middle = Position::new(size.width / 2, size.height / 2);
        let bottom_right = Position::new(size.width, size.height);
        let top_left = Position::new(0, 0);
        let middle_left = Position::new(0, size.height / 2);
        let top_right = Position::new(size.width, 0);
        let top_middle = Position::new(size.width / 2, 0);
        let bottom_left = Position::new(0, size.height);

        canvas.set_draw_color(Color::from_str("green"));
        canvas.draw_line(&middle, &bottom_right);

        canvas.set_draw_color(Color::from_str("white"));
        canvas.draw_line(&top_left, &middle);

        canvas.set_draw_color(Color::from_str("blue"));
        canvas.draw_line(&middle_left, &top_right);

        canvas.set_draw_color(Color::from_str("red"));
        canvas.draw_line(&top_middle, &bottom_left);

        let mut s = String::new();
        write!(s, "{:?}", canvas.buffer)?;

        let expect = "SimpleBuffer (10x10): [\
        [255, 255, 255, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][255, 0, 0, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][0, 0, 255, 255]\
        [100, 100, 100, 100][255, 255, 255, 255][100, 100, 100, 100][100, 100, 100, 100][255, 0, 0, 255][100, 100, 100, 100][100, 100, 100, 100][0, 0, 255, 255][0, 0, 255, 255][100, 100, 100, 100]\
        [100, 100, 100, 100][100, 100, 100, 100][255, 255, 255, 255][100, 100, 100, 100][255, 0, 0, 255][0, 0, 255, 255][0, 0, 255, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100]\
        [100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][255, 0, 0, 255][0, 0, 255, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100]\
        [100, 100, 100, 100][0, 0, 255, 255][0, 0, 255, 255][255, 0, 0, 255][255, 255, 255, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100]\
        [0, 0, 255, 255][100, 100, 100, 100][255, 0, 0, 255][100, 100, 100, 100][100, 100, 100, 100][255, 255, 255, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100]\
        [100, 100, 100, 100][100, 100, 100, 100][255, 0, 0, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][0, 255, 0, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100]\
        [100, 100, 100, 100][255, 0, 0, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][0, 255, 0, 255][100, 100, 100, 100][100, 100, 100, 100]\
        [100, 100, 100, 100][255, 0, 0, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][0, 255, 0, 255][100, 100, 100, 100]\
        [255, 0, 0, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][0, 255, 0, 255]\
        ]";

        assert_eq!(expect, s);
        Ok(())
    }
}
