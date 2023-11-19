use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

use crate::{buffer::Buffer, canvas::Canvas, PhysicalSize, Res};

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

impl Canvas {
    pub fn new_with_pixels(
        buffer_size: winit::dpi::PhysicalSize<u32>,
        window: &Window,
    ) -> Res<Self> {
        let buffer = Pixels::new(
            buffer_size.width,
            buffer_size.height,
            SurfaceTexture::new(buffer_size.width, buffer_size.height, &window),
        )?;
        let buffer = Box::new(buffer);
        let buffer_size = PhysicalSize::new(buffer_size.width, buffer_size.height);
        Ok(Self::new_from_buffer(buffer_size, buffer))
    }
}
