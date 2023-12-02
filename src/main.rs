#![forbid(unconditional_recursion)]
use rand::Rng;
use rusty_tree::{
    camera::Camera,
    canvas::{self, Canvas, Drawable},
    color::Color,
    math_2d::{Intersection, Vector},
    physics_2d::{Body, BodyBuilder, Shape, VisualShape},
    position::Position,
    renderer::Renderer,
    world::World,
    PhysicalSize,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

struct Point {
    color: Color,
    position: Vector,
    velocity: Vector,
}

fn main() {
    let v1 = Vector::new(10.0, 10.0);
    let v2 = Vector::new(-10.0, -10.0);

    let res = v1.intersection(&-&v2, &Vector::zero());
    println!("{:?}", res);
    panic!("");
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new().with_title("RustyTree");
    // .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    let window = builder.build(&event_loop).unwrap();
    let mut size = window.inner_size();

    let renderer = Renderer::new(
        Camera::new(Vector::new(
            size.width as f64 / 2.0,
            size.height as f64 / 2.0,
        )),
        Canvas::new_with_pixels(size, &window).unwrap(),
    );
    let mut world: World = World::new(
        renderer,
        PhysicalSize::new(size.width / 100, size.height / 100),
        1,
    );
    world.renderer.set_draw_color("red".into());

    let body: Body<Vector> = BodyBuilder::new()
        .shape(Shape::Ellipse(Vector::zero(), 100, 200), false)
        .build();
    world.add_body(body, None);

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
                world.renderer.resize(
                    PhysicalSize::new(size.width, size.height),
                    Some(Camera::new(Vector::new(
                        size.width as f64 / 2.0,
                        size.height as f64 / 2.0,
                    ))),
                );
                first = true;
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
                world.render().unwrap();
            }
            _ => (),
        }
    })
}
