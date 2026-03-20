mod drawing;
mod raylib;
use crate::{
    drawing::{cube::Cube, graphical_object::GraphicalObject, sphere::Sphere},
    raylib::{camera::Camera3D, color::Color, input, vector::Vec3, window::Window},
};

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;
const GRID_SIZE: i32 = 10;

fn main() {
    let window = Window::new(SCREEN_WIDTH, SCREEN_HEIGHT, "hello world");
    let mut camera = Camera3D::perspective(
        Vec3 {
            x: 4.0,
            y: 4.0,
            z: 4.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        45.0,
    );

    let mut graphical_objects = vec![];

    let cube = Cube {
        position: Vec3 {
            x: -1.0,
            y: 0.0,
            z: 1.0,
        },
        dimensions: Vec3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
        color: Color::RED,
        wire_color: Color::GREEN,
    };

    let sphere = Sphere {
        position: Vec3 {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        },
        radius: 0.5,
        color: Color::PINK,
        wire_color: Color::GREEN,
    };

    graphical_objects.push(GraphicalObject::Cube(cube));
    graphical_objects.push(GraphicalObject::Sphere(sphere));

    while !window.should_close() {
        let mouse_position = input::get_mouse_position();
        let ray = input::get_screen_to_world_ray(mouse_position, &camera);

        let plane_y = 0.0;
        let t = (plane_y - ray.position.y) / ray.direction.y;

        let ctx = window.drawing_context();

        ctx.clear_background(&Color::BLACK);

        let ctx3d = ctx.drawing_context_3d(&mut camera);

        graphical_objects.iter_mut().for_each(|thing| {
            if t > 0.0 && ray.direction.y != 0.0 {
                thing.update_from_ray(&ray, t);
            }
            thing.draw(&ctx3d);
        });

        ctx3d.draw_grid(GRID_SIZE, 1.0);
    }
}

// Local Variables:
// jinx-local-words: "esc"
// End:
