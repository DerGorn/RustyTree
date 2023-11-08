use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use rusty_tree::{Canvas, Color, Vector};
use std::time::Instant;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

const RADIUS: i32 = 15;
const RADIUS_SQUARE: i32 = RADIUS.pow(2);

fn create_background_buffer(size: PhysicalSize<u32>) -> Canvas {
    let buffer = Canvas::new_with_simplebuffer(size);
    buffer
}

fn reset_background_buffer(buffer: &mut Canvas) {
    buffer.clear(0);

    let (a, b) = (
        (buffer.get_width() as f64 * 0.4) as u32,
        (buffer.get_height() as f64 * 0.4) as u32,
    );

    let origin_x = buffer.get_width() / 2;
    let origin_y = buffer.get_height() / 2;
    buffer.set_draw_color(Color::from_str("white"));
    buffer.draw_ellipse(Vector::new(origin_x, origin_y), a, b);
    buffer.set_draw_color(Color::from_str("red"));
    buffer.draw_ellipse(Vector::new(origin_x, origin_y), b, a);
    buffer.set_fill_color(Color::from_str("blue"));
    buffer.fill_rect(Vector::new(origin_x - 50, origin_y - 50), 100, 100);
    buffer.set_fill_color(Color::from_str("red"));
    buffer.fill_ellipse(Vector::new(origin_x, origin_y), 30, 30);
    buffer.set_draw_color(Color::from_str("green"));
    buffer.draw_line(
        Vector::new(origin_x, origin_y),
        Vector::new(buffer.get_width(), buffer.get_height()),
    );
    buffer.set_draw_color(Color::from_str("red"));
    buffer.draw_line(
        Vector::new(origin_x, 0),
        Vector::new(0, buffer.get_height()),
    );
    buffer.set_draw_color(Color::from_str("blue"));
    buffer.draw_line(Vector::new(0, origin_y), Vector::new(buffer.get_width(), 0));
    buffer.set_draw_color(Color::from_str("white"));
    buffer.draw_line(Vector::new(origin_x, origin_y), Vector::new(0, 0));
    buffer.set_draw_color(Color::from_str("white"));
    buffer.draw_rect(Vector::new(origin_x - 50, origin_y - 50), 100, 100);
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

    let mut background_buffer: Canvas = create_background_buffer(size);
    reset_background_buffer(&mut background_buffer);
    let mut buffer = Canvas::new_with_pixels(size, &window).unwrap();

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
                background_buffer = create_background_buffer(size);
                reset_background_buffer(&mut background_buffer);

                buffer.resize(size);
                buffer
                    .as_slice()
                    .clone_from_slice(&background_buffer.as_slice());
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

                let frame = buffer.as_slice();

                // let elapsed = start.elapsed();
                // println!("Debug: {:?}", elapsed);

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
