use crate::raylib::drawing_context_3d::DrawingContext3D;

pub trait Drawable {
    fn draw(&self, ctx: &DrawingContext3D);
}
