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

        cube_position.x += 0.01 * delta.x;
        cube_position.y += 0.01 * -delta.y;

        let ctx = window.drawing_context();

        ctx.clear_background(&Color::BLACK);

        ctx.draw_text(format!("delta: {}", delta), 0, 0, 16, &Color::BLUE);

        let ctx3d = ctx.drawing_context_3d(&mut camera);

        ctx3d.draw_cube(cube_position, 2.0, 2.0, 2.0, &Color::RED);

        ctx3d.draw_cube_wires(cube_position, 2.0, 2.0, 2.0, &Color::GREEN);

        ctx3d.draw_grid(GRID_SIZE, 1.0);
    }
}

// Local Variables:
// jinx-local-words: "esc"
// End:
