use std::ffi::{c_char, c_int, CString};

use super::{
    ffi::{
        CloseWindow, GetFrameTime, GetMouseDelta, GetMousePosition, GetScreenToWorldRay, GetTime,
        InitWindow, IsKeyDown, IsWindowReady, KeyboardKey_KEY_DOWN, KeyboardKey_KEY_LEFT,
        KeyboardKey_KEY_RIGHT, KeyboardKey_KEY_UP, WindowShouldClose,
    },
    Camera3D, DrawingContext, Ray, Vec2,
};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
#[allow(unused)]
pub enum KeyboardKey {
    KeyUp = 0,
    KeyDown = 1,
    KeyLeft = 2,
    KeyRight = 3,
}

#[non_exhaustive]
pub struct Window;

#[allow(unused)]
impl Window {
    pub fn new(width: i32, height: i32, title: &str) -> Result<Self, ()> {
        unsafe {
            InitWindow(
                width,
                height,
                CString::new(title).unwrap().as_ptr() as *const c_char,
            );

            if !IsWindowReady() {
                Err(())
            } else {
                Ok(Self)
            }
        }
    }

    pub fn should_close(&self) -> bool {
        unsafe { WindowShouldClose() }
    }

    pub fn drawing_context(&self) -> DrawingContext {
        DrawingContext::new()
    }

    pub fn is_key_down(key: KeyboardKey) -> bool {
        unsafe {
            match key {
                KeyboardKey::KeyUp => IsKeyDown(KeyboardKey_KEY_UP as c_int),
                KeyboardKey::KeyDown => IsKeyDown(KeyboardKey_KEY_DOWN as c_int),
                KeyboardKey::KeyLeft => IsKeyDown(KeyboardKey_KEY_LEFT as c_int),
                KeyboardKey::KeyRight => IsKeyDown(KeyboardKey_KEY_RIGHT as c_int),
            }
        }
    }

    pub fn get_mouse_delta(&self) -> Vec2<f32> {
        unsafe { GetMouseDelta().into() }
    }

    pub fn get_mouse_position(&self) -> Vec2<f32> {
        unsafe { GetMousePosition().into() }
    }

    pub fn get_screen_to_world_ray(&self, mouse_position: Vec2<f32>, camera: &Camera3D) -> Ray {
        unsafe { GetScreenToWorldRay(mouse_position.into(), (*camera).into()).into() }
    }

    pub fn get_frame_time(&self) -> f32 {
        unsafe { GetFrameTime() }
    }

    pub fn get_time(&self) -> f64 {
        unsafe { GetTime() }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { CloseWindow() }
    }
}
