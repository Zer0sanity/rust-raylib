use crate::{
    drawing::drawable::Drawable,
    raylib::{Color, DrawingContext3D, Vec3},
};

pub struct Cube {
    pub position: Vec3<f32>,
    pub dimensions: Vec3<f32>,
    pub color: Color,
    pub wire_color: Color,
}

impl Drawable for Cube {
    fn draw(&self, ctx: &DrawingContext3D) {
        ctx.draw_cube(
            self.position,
            self.dimensions.x,
            self.dimensions.z,
            self.dimensions.y,
            &self.color,
        );
        ctx.draw_cube_wires(
            self.position,
            self.dimensions.x,
            self.dimensions.z,
            self.dimensions.y,
            &self.wire_color,
        );
    }
}
