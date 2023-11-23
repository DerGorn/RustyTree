use std::hash::Hash;
use std::{cell::RefCell, error::Error, rc::Rc};

use math_2d::Vector;
use physics_2d::{Body, CollisionBody, VisiualShape};

#[derive(Debug, Clone, Copy)]
pub struct PhysicalSize<P> {
    pub width: P,
    pub height: P,
}
impl<P> PhysicalSize<P> {
    pub fn new(width: P, height: P) -> Self {
        Self { width, height }
    }
}

pub mod math_2d;

pub mod color;

pub mod buffer;

pub mod canvas;

pub mod position;

pub mod camera;

pub mod renderer;

mod spatial_hashgrid;

pub mod world;

pub mod physics_2d;

#[cfg(feature = "pixels")]
pub mod pixels_integration;

type Res<T, E = Box<dyn Error>> = Result<T, E>;
