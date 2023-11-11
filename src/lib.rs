use std::error::Error;

mod math_2d;
pub use math_2d::Matrix;
pub use math_2d::Vector;

mod color;
pub use color::Color;

mod buffer;
pub use buffer::Buffer;

mod canvas;
pub use canvas::Canvas;
pub use canvas::Drawable;

mod collision;

mod position;
pub use position::Position;

mod camera;
pub use camera::Camera;

mod renderer;
pub use renderer::Renderer;

type Res<T, E = Box<dyn Error>> = Result<T, E>;
