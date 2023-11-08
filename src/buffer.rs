use pixels::Pixels;
use winit::dpi::PhysicalSize;

use crate::Res;

pub trait Buffer {
    fn buffer<'a>(&'a mut self) -> &'a mut [u8];

    fn clear(&mut self, clear_value: u8) {
        self.buffer().fill(clear_value);
    }

    fn resize(&mut self, buffer_size: PhysicalSize<u32>);

    fn render(&self) -> Res<()>;
}

pub struct SimpleBuffer {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
}
impl Buffer for SimpleBuffer {
    fn buffer<'a>(&'a mut self) -> &'a mut [u8] {
        &mut self.buffer
    }

    fn resize(&mut self, buffer_size: PhysicalSize<u32>) {
        self.buffer = vec![0; (buffer_size.width * buffer_size.height * 4) as usize];
    }

    fn render(&self) -> Res<()> {
        Err("Trying to render a SimpleBuffer, but that doesnt know where to render".into())
    }
}
impl SimpleBuffer {
    pub fn new(buffer_size: PhysicalSize<u32>) -> Self {
        Self {
            width: buffer_size.width,
            height: buffer_size.height,
            buffer: vec![0; (buffer_size.width * buffer_size.height * 4) as usize],
        }
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> Res<&mut [u8; 4]> {
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

impl Buffer for Pixels {
    fn buffer<'a>(&'a mut self) -> &'a mut [u8] {
        self.frame_mut()
    }

    fn resize(&mut self, buffer_size: PhysicalSize<u32>) {
        self.resize_surface(buffer_size.width, buffer_size.height)
            .unwrap();
        self.resize_buffer(buffer_size.width, buffer_size.height)
            .unwrap();
    }

    fn render(&self) -> Res<()> {
        self.render()?;
        Ok(())
    }
}
