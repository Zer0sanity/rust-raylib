use std::ffi::{c_char, CString};

use super::{
    ffi::{
        BeginDrawing, ClearBackground, DrawRectangle, DrawRectangleLines, DrawText, EndDrawing,
        MeasureText,
    },
    Camera3D, Color, DrawingContext3D,
};

#[non_exhaustive]
pub struct DrawingContext;

#[allow(unused)]
impl DrawingContext {
    pub fn new() -> Self {
        unsafe { BeginDrawing() }
        Self
    }

    pub fn clear_background(&self, color: &Color) {
        unsafe { ClearBackground(color.into()) }
    }

    pub fn draw_rectangle(&self, x: i32, y: i32, width: i32, height: i32, color: &Color) {
        unsafe {
            DrawRectangle(x, y, width, height, color.into());
        }
    }

    pub fn draw_rectangle_lines(&self, x: i32, y: i32, width: i32, height: i32, color: &Color) {
        unsafe {
            DrawRectangleLines(x, y, width, height, color.into());
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
            DrawText(
                CString::new(text).unwrap().as_ptr() as *const c_char,
                x,
                y,
                font_size,
                color.into(),
            );
        }
    }

    pub fn measure_text<T: Into<Vec<u8>>>(&self, text: T, font_size: i32) -> i32 {
        unsafe {
            MeasureText(
                CString::new(text).unwrap().as_ptr() as *const c_char,
                font_size,
            )
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
        let text_size = self.measure_text(text.clone(), font_size);
        self.draw_text(text, x - text_size / 2, y, font_size, color.into());
    }

    pub fn drawing_context_3d(&self, camera: &mut Camera3D) -> DrawingContext3D {
        DrawingContext3D::new(camera)
    }
}

impl Drop for DrawingContext {
    fn drop(&mut self) {
        unsafe { EndDrawing() }
    }
}
