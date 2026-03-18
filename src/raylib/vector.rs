use std::fmt::{self, Display};

use super::bindings::ffi::{Vector2, Vector3};

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl From<Vec2<f32>> for Vector2 {
    fn from(v: Vec2<f32>) -> Vector2 {
        Vector2 { x: v.x, y: v.y }
    }
}

impl From<Vector2> for Vec2<f32> {
    fn from(v: Vector2) -> Vec2<f32> {
        Vec2 { x: v.x, y: v.y }
    }
}

impl Display for Vec2<f32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl From<Vec3<f32>> for Vector3 {
    fn from(v: Vec3<f32>) -> Vector3 {
        Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
