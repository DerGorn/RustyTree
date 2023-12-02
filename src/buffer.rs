use std::fmt::Debug;

use crate::{PhysicalSize, Res};

pub trait Buffer: Debug {
    fn buffer<'a>(&'a mut self) -> &'a mut [u8];

    fn clear(&mut self, clear_value: [u8; 4]) {
        let mut i = 0;
        self.buffer().fill_with(|| {
            let v = clear_value[i];
            i = (i + 1) % 4;
            v
        });
    }

    fn resize(&mut self, buffer_size: PhysicalSize<u32>);

    fn render(&self) -> Res<()>;
}

pub struct SimpleBuffer {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
}
impl SimpleBuffer {
    pub fn new(buffer_size: PhysicalSize<u32>) -> Self {
        Self {
            width: buffer_size.width,
            height: buffer_size.height,
            buffer: vec![0; (buffer_size.width * buffer_size.height * 4) as usize],
        }
    }

    fn get_pixel(&mut self, x: usize, y: usize) -> Res<&mut [u8; 4]> {
        let index = (self.width as usize * y + x) * 4;
        match self.buffer.get_mut(index..index + 4) {
            None => Err(format!(
                "Index out of bounds. Buffer is only {} bytes big, but index is {}",
                self.width * self.height * 4,
                index
            )
            .into()),
            Some(pixel) => Ok(pixel.try_into()?),
        }
    }
}

impl Debug for SimpleBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = format!("SimpleBuffer ({}x{}): [", self.width, self.height);
        for pixel in self.buffer.chunks(4) {
            res += format!("{:?}", pixel).as_str();
        }
        write!(f, "{}]", res)
    }
}

impl Buffer for SimpleBuffer {
    fn buffer<'a>(&'a mut self) -> &'a mut [u8] {
        &mut self.buffer
    }

    fn resize(&mut self, buffer_size: PhysicalSize<u32>) {
        self.buffer = vec![0; (buffer_size.width * buffer_size.height * 4) as usize];
        self.width = buffer_size.width;
        self.height = buffer_size.height;
    }

    fn render(&self) -> Res<()> {
        Err("SimpleBuffer tries to render, but it cannot do that. This is only a SimpleBuffer to hold data. If you want to render it, feed the `buffer()` somewhere that can render or implement your own Buffer.".into())
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    use super::*;
    use std::fmt::Write;

    #[test]
    fn buffer_length() {
        let w = 1;
        let h = 2;
        let buffer = SimpleBuffer::new(PhysicalSize::new(w, h));

        assert!(buffer.buffer.len() as u32 == w * h * 4)
    }

    #[test]
    fn debug() -> Res<()> {
        let buffer = SimpleBuffer::new(PhysicalSize::new(1, 2));

        let mut s = String::new();
        write!(s, "{:?}", buffer)?;

        assert_eq!("SimpleBuffer (1x2): [[0, 0, 0, 0][0, 0, 0, 0]]", s);

        Ok(())
    }

    #[test]
    fn get_pixel() {
        let mut buffer = SimpleBuffer::new(PhysicalSize::new(1, 2));
        buffer.buffer()[4] = 255;

        let pixel = buffer.get_pixel(0, 1).unwrap();

        assert_eq!(pixel[0], 255);
        assert_eq!(pixel[1], 0);
        assert_eq!(pixel[2], 0);
        assert_eq!(pixel[3], 0);
    }

    #[test]
    fn clear() {
        let mut buffer = SimpleBuffer::new(PhysicalSize::new(1, 2));
        buffer.clear(Color::new_rgba(200, 200, 200, 200).to_slice());

        assert!(buffer.buffer().iter().all(|v| *v == 200_u8))
    }

    #[test]
    fn resize() -> Res<()> {
        let mut buffer = SimpleBuffer::new(PhysicalSize::new(1, 1));
        buffer.resize(PhysicalSize::new(1, 2));

        let mut s = String::new();
        write!(s, "{:?}", buffer)?;

        assert_eq!("SimpleBuffer (1x2): [[0, 0, 0, 0][0, 0, 0, 0]]", s);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "feed the `buffer()` somewhere that can render")]
    fn render() {
        let buffer = SimpleBuffer::new(PhysicalSize::new(1, 2));

        buffer.render().unwrap()
    }
}
