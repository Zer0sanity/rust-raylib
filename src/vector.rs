use crate::raylib::ffi::Vector3;

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
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
