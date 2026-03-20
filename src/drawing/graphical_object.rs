use crate::{
    drawing::{cube::Cube, drawable::Drawable, sphere::Sphere},
    raylib::{DrawingContext3D, Ray},
};

pub enum GraphicalObject {
    Cube(Cube),
    Sphere(Sphere),
}

impl GraphicalObject {
    pub fn draw(&self, ctx: &DrawingContext3D) {
        match self {
            Self::Cube(c) => c.draw(ctx),
            Self::Sphere(s) => s.draw(ctx),
        }
    }

    pub fn update_from_ray(&mut self, ray: &Ray, t: f32) {
        match self {
            Self::Cube(c) => {
                c.position.x = -(ray.position.x + ray.direction.x * t);
                c.position.z = ray.position.z + ray.direction.z * t;
            }
            Self::Sphere(s) => {
                s.position.x = ray.position.x + ray.direction.x * t;
                s.position.z = -(ray.position.z + ray.direction.z * t);
            }
        }
    }
}
