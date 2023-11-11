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
        (buffer.get_width() as f64 * 0.4) as u32,
        (buffer.get_height() as f64 * 0.4) as u32,
    );

    let center = Vector::zero();
    buffer.set_draw_color(Color::from_str("white"));
    buffer.draw_ellipse(&center, a, b, 0.0);
    buffer.set_draw_color(Color::from_str("red"));
    buffer.draw_ellipse(&center, a, b, 95.0);
    buffer.set_draw_color(Color::from_str("green"));
    buffer.draw_ellipse(&center, a, b, 190.0);
    buffer.set_draw_color(Color::from_str("blue"));
    buffer.draw_ellipse(&center, b, a, 10.0);
    // buffer.set_fill_color(Color::from_str("blue"));
    // buffer.fill_ellipse_old(&center, 200, 300, 200.0);
    buffer.set_fill_color(Color::from_str("whine_red"));
    buffer.fill_ellipse(&center, 200, 300, 110.0);
    // buffer.set_draw_color(Color::from_str("purple"));
    // buffer.draw_ellipse(&center, a, b, 380.0);
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
                window.request_redraw();
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
