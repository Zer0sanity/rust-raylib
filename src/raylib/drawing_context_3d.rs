use super::{
    ffi::{BeginMode3D, DrawCube, DrawCubeWires, DrawGrid, DrawSphere, DrawSphereWires, EndMode3D},
    Camera3D, Color, Vec3,
};

#[non_exhaustive]
pub struct DrawingContext3D;

#[allow(unused)]
impl DrawingContext3D {
    pub fn new(camera: &Camera3D) -> Self {
        unsafe {
            BeginMode3D((*camera).into());
        }
        Self
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
            DrawCube(position.into(), width, height, length, color.into());
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
            DrawCubeWires(position.into(), width, height, length, color.into());
        }
    }

    pub fn draw_sphere(&self, position: Vec3<f32>, radius: f32, color: &Color) {
        unsafe {
            DrawSphere(position.into(), radius, color.into());
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
            DrawSphereWires(position.into(), radius, rings, slices, color.into());
        }
    }

    pub fn draw_grid(&self, slices: i32, spacing: f32) {
        unsafe {
            DrawGrid(slices, spacing);
        }
    }
}

impl Drop for DrawingContext3D {
    fn drop(&mut self) {
        unsafe {
            EndMode3D();
        }
    }
}
