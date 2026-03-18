use std::ffi::{c_char, CString};

use super::bindings::ffi;
use super::camera::Camera3D;
use super::color::Color;
use super::drawing_context_3d::DrawingContext3D;
use super::helpers::measure_text;

pub struct DrawingContext;

impl DrawingContext {
    pub fn new() -> Self {
        unsafe { ffi::BeginDrawing() }

        Self {}
    }

    pub fn clear_background(&self, color: &Color) {
        unsafe { ffi::ClearBackground(color.into()) }
    }

    pub fn draw_rectangle(&self, x: i32, y: i32, width: i32, height: i32, color: &Color) {
        unsafe {
            ffi::DrawRectangle(x, y, width, height, color.into());
        }
    }

    pub fn draw_rectangle_lines(&self, x: i32, y: i32, width: i32, height: i32, color: &Color) {
        unsafe {
            ffi::DrawRectangleLines(x, y, width, height, color.into());
        }
    }

    pub fn draw_text<T: Into<Vec<u8>>>(
        &self,
        text: T,
        x: i32,
        y: i32,
        font_size: i32,
        color: &Color,
    ) {
        unsafe {
            ffi::DrawText(
                CString::new(text).unwrap().as_ptr() as *const c_char,
                x,
                y,
                font_size,
                color.into(),
            );
        }
    }

    pub fn draw_centered_text<T: Into<Vec<u8>>>(
        &self,
        text: T,
        x: i32,
        y: i32,
        font_size: i32,
        color: &Color,
    ) {
        let text: Vec<u8> = text.into();
        let text_size = measure_text(text.clone(), font_size);
        self.draw_text(text, x - text_size / 2, y, font_size, color.into());
    }

    pub fn drawing_context_3d(&self, camera: &mut Camera3D) -> DrawingContext3D {
        DrawingContext3D::new(camera)
    }
}

impl Drop for DrawingContext {
    fn drop(&mut self) {
        unsafe { ffi::EndDrawing() }
    }
}
