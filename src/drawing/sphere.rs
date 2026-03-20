use crate::{
    drawing::drawable::Drawable,
    raylib::{color::Color, drawing_context_3d::DrawingContext3D, vector::Vec3},
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
