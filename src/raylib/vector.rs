use std::fmt::{self, Display};

use super::ffi::{Vector2, Vector3};

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

impl<T: Display> Display for Vec2<T> {
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

impl From<Vector3> for Vec3<f32> {
    fn from(v: Vector3) -> Vec3<f32> {
        Vec3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, x: {}", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[allow(unused)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
