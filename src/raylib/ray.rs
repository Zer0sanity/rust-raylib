use super::bindings::ffi;
use super::vector::Vec3;
use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Ray {
    pub position: Vec3<f32>,
    pub direction: Vec3<f32>,
}

impl From<Ray> for ffi::Ray {
    fn from(r: Ray) -> ffi::Ray {
        ffi::Ray {
            position: r.position.into(),
            direction: r.direction.into(),
        }
    }
}

impl From<ffi::Ray> for Ray {
    fn from(r: ffi::Ray) -> Ray {
        Ray {
            position: r.position.into(),
            direction: r.direction.into(),
        }
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "position: {}, direction: {}",
            self.position, self.direction
        )
    }
}
