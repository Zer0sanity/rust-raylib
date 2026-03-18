use std::ffi::{c_char, CString};

use crate::raylib::drawing_context::DrawingContext;

use super::bindings::ffi;

pub struct Window;

impl Window {
    pub fn new(width: i32, height: i32, title: &str) -> Self {
        unsafe {
            ffi::InitWindow(
                width,
                height,
                CString::new(title).unwrap().as_ptr() as *const c_char,
            );
        }

        Self {}
    }

    pub fn is_window_ready() -> bool {
        unsafe { ffi::IsWindowReady() }
    }

    pub fn should_close(&self) -> bool {
        unsafe { ffi::WindowShouldClose() }
    }

    pub fn drawing_context(&self) -> DrawingContext {
        DrawingContext::new()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { ffi::CloseWindow() }
    }
}
