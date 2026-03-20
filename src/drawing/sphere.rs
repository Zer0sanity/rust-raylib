use crate::{
    drawing::drawable::Drawable,
    raylib::{Color, DrawingContext3D, Vec3},
};

pub struct Sphere {
    pub position: Vec3<f32>,
    pub radius: f32,
    pub color: Color,
    pub wire_color: Color,
}

impl Drawable for Sphere {
    fn draw(&self, ctx: &DrawingContext3D) {
        ctx.draw_sphere(self.position, self.radius, &self.color);
        ctx.draw_sphere_wires(self.position, self.radius, 16, 16, &self.wire_color);
    }
}
