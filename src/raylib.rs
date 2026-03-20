// #[allow(warnings, unused, clippy::approx_constant)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/raylib_bindings.rs"));
}

mod camera;

mod color;
mod drawing_context;
mod drawing_context_3d;
mod ray;
mod vector;
mod window;

pub use camera::Camera3D;
pub use color::Color;
pub use drawing_context::DrawingContext;
pub use drawing_context_3d::DrawingContext3D;
pub use ray::Ray;
pub use vector::Vec2;
pub use vector::Vec3;
pub use window::Window;
