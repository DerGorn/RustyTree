use std::time::Instant;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::VirtualKeyCode,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

const RADIUS: i32 = 15;
const RADIUS_SQUARE: i32 = RADIUS.pow(2);

fn main() {
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new().with_title("RustyTree");
    // .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    let window = builder.build(&event_loop).unwrap();
    let mut size = window.inner_size();

    let mut background_buffer: Vec<u8> = vec![200; (size.width * size.height * 4) as usize];
    let mut buffer = Pixels::new(
        size.width,
        size.height,
        SurfaceTexture::new(size.width, size.height, &window),
    )
    .unwrap();

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let (mut old_x, mut old_y) = (0, 0);
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(s),
                ..
            } => {
                size = s;
                background_buffer = vec![200; (size.width * size.height * 4) as usize];
                buffer.resize_surface(size.width, size.height).unwrap();
                buffer.resize_buffer(size.width, size.height).unwrap();
                buffer.frame_mut().clone_from_slice(&background_buffer);
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                (x, y) = ((position.x).ceil() as i32, (position.y).ceil() as i32);
            }
            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                let start = Instant::now();

                let frame = buffer.frame_mut();
                // frame.clone_from_slice(&background_buffer);
                for x_offset in -RADIUS..=RADIUS {
                    let x = ((old_x as i32) + x_offset) as u32;
                    if x >= size.width {
                        continue;
                    }
                    let height = ((RADIUS_SQUARE - x_offset.pow(2)) as f64).sqrt() as i32;
                    for y_offset in -height..=height {
                        let y = ((old_y as i32) + y_offset) as u32;
                        if y >= size.height {
                            continue;
                        }

                        let index = ((size.width * y + x) * 4) as usize;
                        frame[index] = background_buffer[index];
                        frame[index + 1] = background_buffer[index + 1];
                        frame[index + 2] = background_buffer[index + 2];
                        frame[index + 3] = background_buffer[index + 3];
                    }
                }
                (old_x, old_y) = (x, y);
                for x_offset in -RADIUS..=RADIUS {
                    let x: u32 = ((x as i32) + x_offset) as u32;
                    if x >= size.width {
                        continue;
                    }
                    let height = ((RADIUS_SQUARE - x_offset.pow(2)) as f64).sqrt() as i32;
                    for y_offset in -height..=height {
                        let y = ((y as i32) + y_offset) as u32;
                        if y >= size.height {
                            continue;
                        }

                        let index = ((size.width * y + x) * 4) as usize;
                        frame[index] = 255;
                        frame[index + 1] = 255;
                        frame[index + 2] = 255;
                        frame[index + 3] = 255;
                    }
                }
                
                let elapsed = start.elapsed();
                println!("Debug: {:?}", elapsed);

                // for pixel in frame.chunks_exact_mut(4) {
                //     pixel[0] = 0xff; // R
                //     pixel[1] = 0xff; // G
                //     pixel[2] = 0xff; // B
                //     pixel[3] = 0xff; // A
                // }
                buffer.render().unwrap();
            }
            _ => (),
        }
    })
}
