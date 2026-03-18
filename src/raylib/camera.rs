use super::bindings::ffi;
use super::vector::Vec3;
use std::mem;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera3D {
    pub position: Vec3<f32>,
    pub target: Vec3<f32>,
    pub up: Vec3<f32>,
    pub fovy: f32,
    projection_: ffi::CameraProjection,
}

impl Camera3D {
    pub fn perspective(
        position: Vec3<f32>,
        target: Vec3<f32>,
        up: Vec3<f32>,
        fovy: f32,
    ) -> Camera3D {
        Camera3D {
            position,
            target,
            up,
            fovy,
            projection_: ffi::CameraProjection_CAMERA_PERSPECTIVE,
        }
    }

    pub fn orthographic(
        position: Vec3<f32>,
        target: Vec3<f32>,
        up: Vec3<f32>,
        fovy: f32,
    ) -> Camera3D {
        let mut c = Self::perspective(position, target, up, fovy);
        c.projection_ = ffi::CameraProjection_CAMERA_ORTHOGRAPHIC;
        c
    }
}

impl From<ffi::Camera3D> for Camera3D {
    fn from(v: ffi::Camera3D) -> Camera3D {
        unsafe { mem::transmute(v) }
    }
}

impl Into<ffi::Camera3D> for Camera3D {
    fn into(self) -> ffi::Camera3D {
        unsafe { mem::transmute(self) }
    }
}

impl Into<ffi::Camera3D> for &Camera3D {
    fn into(self) -> ffi::Camera3D {
        ffi::Camera3D {
            position: self.position.into(),
            target: self.target.into(),
            up: self.up.into(),
            fovy: self.fovy,
            projection: (self.projection_ as u32) as i32,
        }
    }
}
