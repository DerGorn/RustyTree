use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::VirtualKeyCode,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new().with_title("RustyTree");
    // .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    let window = builder.build(&event_loop).unwrap();
    let size = window.inner_size();

    let mut buffer = Pixels::new(
        size.width,
        size.height,
        SurfaceTexture::new(size.width, size.height, &window),
    )
    .unwrap();

    let mut i = 0;
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
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
                let frame = buffer.frame_mut();
                for pixel in frame.chunks_exact_mut(4) {
                    pixel[0] = 0xff; // R
                    pixel[1] = 0xff; // G
                    pixel[2] = 0xff; // B
                    pixel[3] = 0xff; // A
                }
                buffer.render().unwrap();
            }
            _ => (),
        }
    })
}
