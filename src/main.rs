use std::borrow::BorrowMut;

use rand::Rng;
use rusty_tree::{Camera, Canvas, Color, Drawable, Renderer, Vector};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn reset_background_buffer(buffer: &mut Renderer) {
    buffer.clear(0);

    let (a, b) = (
        (buffer.get_width() as f64 * 0.25) as u32,
        (buffer.get_height() as f64 * 0.25) as u32,
    );

    buffer.set_draw_color(Color::from_str("red"));
    let center = Vector::zero();
    for deg in (0..=180).step_by(10) {
        let deg = deg as f64;
        buffer.set_fill_color(Color::new_hsva(
            (deg * 255.0 / 180.0).round() as u8,
            255,
            255,
            20,
        ));
        buffer.fill_rect(&center, a*2, b*2, deg);
    }
}

struct Point {
    color: Color,
    position: Vector,
    velocity: Vector,
}

fn main() {
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new().with_title("RustyTree");
    // .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    let window = builder.build(&event_loop).unwrap();
    let mut size = window.inner_size();

    let mut renderer = Renderer::new(
        Camera::new(Vector::new(
            size.width as f64 / 2.0,
            size.height as f64 / 2.0,
        )),
        Canvas::new_with_pixels(size, &window).unwrap(),
    );
    reset_background_buffer(&mut renderer);

    let mut first = true;
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
                renderer.resize(
                    size,
                    Some(Camera::new(Vector::new(
                        size.width as f64 / 2.0,
                        size.height as f64 / 2.0,
                    ))),
                );
                first = true;
                reset_background_buffer(&mut renderer);
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {}
            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                if first {
                    first = false;
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // let start = Instant::now();

                // let elapsed = start.elapsed();
                // println!("Debug: {:?}", elapsed);

                // for pixel in frame.chunks_exact_mut(4) {
                //     pixel[0] = 0xff; // R
                //     pixel[1] = 0xff; // G
                //     pixel[2] = 0xff; // B
                //     pixel[3] = 0xff; // A
                // }
                renderer.render().unwrap();
            }
            _ => (),
        }
    })
}
