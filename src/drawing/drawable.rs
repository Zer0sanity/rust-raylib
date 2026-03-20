use crate::raylib::DrawingContext3D;

pub trait Drawable {
    fn draw(&self, ctx: &DrawingContext3D);
}
