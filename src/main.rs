mod raylib;

use crate::raylib::{camera::Camera3D, color::Color, input, vector::Vec3, window::Window};

const SCREEN_WIDTH: i32 = 600;
const SCREEN_HEIGHT: i32 = 600;
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

    let mut cube_position = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    // <esc> will set this to true
    while !window.should_close() {
        let delta = input::get_mouse_delta();

        let mouse_position = input::get_mouse_position();
        let ray = input::get_screen_to_world_ray(mouse_position, &camera);

        cube_position.x += -delta.x / SCREEN_WIDTH as f32;
        cube_position.y += -delta.y / SCREEN_HEIGHT as f32;

        let ctx = window.drawing_context();

        ctx.clear_background(&Color::BLACK);

        ctx.draw_text(format!("ray: {}", cube_position), 0, 0, 16, &Color::BLUE);

        let ctx3d = ctx.drawing_context_3d(&mut camera);

        ctx3d.draw_cube(cube_position, 2.0, 2.0, 2.0, &Color::RED);

        ctx3d.draw_cube_wires(cube_position, 2.0, 2.0, 2.0, &Color::GREEN);

        ctx3d.draw_grid(GRID_SIZE, 1.0);
    }
}

// Local Variables:
// jinx-local-words: "esc"
// End:
