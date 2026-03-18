use std::ffi::{c_char, CString};

use super::bindings::ffi;

pub fn measure_text<T: Into<Vec<u8>>>(text: T, font_size: i32) -> i32 {
    unsafe {
        ffi::MeasureText(
            CString::new(text).unwrap().as_ptr() as *const c_char,
            font_size,
        )
    }
}

pub fn get_frame_time() -> f32 {
    unsafe { ffi::GetFrameTime() }
}

pub fn get_time() -> f64 {
    unsafe { ffi::GetTime() }
}
