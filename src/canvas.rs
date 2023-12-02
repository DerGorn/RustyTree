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

    fn clear(&mut self, clear_value: [u8; 4]);

    fn set_pixel(&mut self, position: &T, color: &Color);

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

    fn set_pixel(&mut self, position: &Position, color: &Color) {
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

    fn clear(&mut self, clear_value: [u8; 4]) {
        self.buffer.clear(clear_value)
    }

    fn draw_line(&mut self, start: &Position, end: &Position) {
        let width = self.size.width - 1;
        let height = self.size.height - 1;
        let buffer = self.buffer.buffer();
        let color = self.draw_color.to_slice();

        let (mut start, mut end) = if start.x > end.x {
            (end, start)
        } else {
            (start, end)
        };

        let run = end.x as f64 - start.x as f64;
        let rise = end.y as f64 - start.y as f64;
        let mut slope = rise / run;
        if slope == f64::INFINITY {
            slope = f64::NEG_INFINITY;
            (start, end) = (end, start);
        }
        let slope_direction = slope.signum() as i32;
        let mut last_y = start.y;

        let (y_min, y_max) = if start.y > end.y {
            (end.y.max(0), start.y.min(height))
        } else {
            (start.y.max(0), end.y.min(height))
        };

        for x in start.x.max(0)..=end.x.min(width) {
            let end = (start.y as f64 + ((x - start.x) as f64 * slope).round()) as u32;
            let (mut start_y, mut end_y) = if last_y > end {
                (end, last_y)
            } else {
                (last_y, end)
            };
            if end_y > start_y && x != start.x {
                if slope_direction == -1 {
                    end_y = (end_y as i32 + slope_direction) as u32;
                } else {
                    start_y = (start_y as i32 + slope_direction) as u32;
                }
            }
            for y in start_y.max(y_min)..=end_y.min(y_max) {
                Self::fill_pixel(buffer, x, y, self.size.width, color);
            }
            last_y = end;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math_2d::Vector;

    use super::*;
    use std::fmt::Write;

    #[test]
    pub fn draw_line() -> Res<()> {
        let size = PhysicalSize::new(10, 10);
        let mut canvas = Canvas::new_with_simplebuffer(size);
        canvas.clear(Color::new_rgba(100, 100, 100, 100).to_slice());

        let middle = Position::new(size.width / 2, size.height / 2);
        let bottom_right = Position::new(size.width - 1, size.height - 1);
        let top_left = Position::new(0, 0);
        let middle_left = Position::new(0, size.height / 2);
        let top_right = Position::new(size.width - 1, 0);
        let top_middle = Position::new(size.width / 2, 0);
        let bottom_left = Position::new(0, size.height - 1);

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
        [100, 100, 100, 100][255, 255, 255, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][255, 0, 0, 255][100, 100, 100, 100][0, 0, 255, 255][0, 0, 255, 255][100, 100, 100, 100]\
        [100, 100, 100, 100][100, 100, 100, 100][255, 255, 255, 255][100, 100, 100, 100][255, 0, 0, 255][0, 0, 255, 255][0, 0, 255, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100]\
        [100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][0, 0, 255, 255][255, 0, 0, 255][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100][100, 100, 100, 100]\
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

    #[test]
    #[ignore]
    fn start_end_points() {
        let size = PhysicalSize::new(301, 301);
        let mut canvas = Canvas::new_with_simplebuffer(size);

        let clear_color = Color::from_str("black");
        canvas.set_draw_color(Color::from_str("red"));

        let buffer_width = canvas.get_width();
        for x in 0..size.width {
            for y in 0..size.height {
                let start = Vector::new(x as f64, y as f64).into();
                let end =
                    Vector::new((size.width - x - 1) as f64, (size.height - y - 1) as f64).into();
                let start_index = ((buffer_width * y + x) * 4) as usize;
                let end_index =
                    ((buffer_width * (size.height - y - 1) + (size.width - x - 1)) * 4) as usize;

                canvas.set_pixel(&start, &clear_color);
                canvas.set_pixel(&end, &clear_color);

                canvas.draw_line(&start, &end);

                {
                    let buffer = canvas.as_slice();
                    println!("\nstart: {} : {}", start, start_index);
                    println!("end: {} : {}", end, end_index);
                    assert_eq!(buffer[start_index], 255, "start failed");
                    assert_eq!(buffer[end_index], 255, "end_failed");
                }

                println!("Reverse");
                canvas.set_pixel(&start, &clear_color);
                canvas.set_pixel(&end, &clear_color);

                canvas.draw_line(&end, &start);

                let buffer = canvas.as_slice();
                assert_eq!(buffer[start_index], 255, "reverse start (end) failed");
                assert_eq!(buffer[end_index], 255, "reverse end (start) failed");
            }
        }
    }
}
