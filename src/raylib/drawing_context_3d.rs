use super::bindings::ffi;
use super::camera::Camera3D;
use super::color::Color;
use super::vector::Vec3;

pub struct DrawingContext3D;

impl DrawingContext3D {
    pub fn new(camera: &Camera3D) -> Self {
        unsafe {
            ffi::BeginMode3D((*camera).into());
        }
        Self {}
    }

    pub fn draw_cube(
        &self,

        position: Vec3<f32>,
        width: f32,
        height: f32,
        length: f32,
        color: &Color,
    ) {
        unsafe {
            ffi::DrawCube(position.into(), width, height, length, color.into());
        }
    }

    pub fn draw_cube_wires(
        &self,
        position: Vec3<f32>,
        width: f32,
        height: f32,
        length: f32,
        color: &Color,
    ) {
        unsafe {
            ffi::DrawCubeWires(position.into(), width, height, length, color.into());
        }
    }

    pub fn draw_sphere(&self, position: Vec3<f32>, radius: f32, color: &Color) {
        unsafe {
            ffi::DrawSphere(position.into(), radius, color.into());
        }
    }

    pub fn draw_sphere_wires(
        &self,
        position: Vec3<f32>,
        radius: f32,
        rings: i32,
        slices: i32,
        color: &Color,
    ) {
        unsafe {
            ffi::DrawSphereWires(position.into(), radius, rings, slices, color.into());
        }
    }

    pub fn draw_grid(&self, slices: i32, spacing: f32) {
        unsafe {
            ffi::DrawGrid(slices, spacing);
        }
    }
}

impl Drop for DrawingContext3D {
    fn drop(&mut self) {
        unsafe {
            ffi::EndMode3D();
        }
    }
}
