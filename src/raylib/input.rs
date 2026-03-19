use super::bindings::ffi;
use super::camera::Camera3D;
use super::ray::Ray;
use super::vector::Vec2;
use std::ffi::c_int;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum KeyboardKey {
    KeyUp = 0,
    KeyDown = 1,
    KeyLeft = 2,
    KeyRight = 3,
}

pub fn is_key_down(key: KeyboardKey) -> bool {
    unsafe {
        match key {
            KeyboardKey::KeyUp => ffi::IsKeyDown(ffi::KeyboardKey_KEY_UP as c_int),
            KeyboardKey::KeyDown => ffi::IsKeyDown(ffi::KeyboardKey_KEY_DOWN as c_int),
            KeyboardKey::KeyLeft => ffi::IsKeyDown(ffi::KeyboardKey_KEY_LEFT as c_int),
            KeyboardKey::KeyRight => ffi::IsKeyDown(ffi::KeyboardKey_KEY_RIGHT as c_int),
        }
    }
}

pub fn get_mouse_delta() -> Vec2<f32> {
    unsafe { ffi::GetMouseDelta().into() }
}

pub fn get_mouse_position() -> Vec2<f32> {
    unsafe { ffi::GetMousePosition().into() }
}

pub fn get_screen_to_world_ray(mouse_position: Vec2<f32>, camera: &Camera3D) -> Ray {
    unsafe { ffi::GetScreenToWorldRay(mouse_position.into(), (*camera).into()).into() }
}
