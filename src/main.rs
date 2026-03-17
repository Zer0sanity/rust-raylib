mod camera;
mod raylib;
mod vector;
mod window;

use raylib::Color;
use vector::Vec3;
use window::Window;

use crate::camera::Camera3D;

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

    let cube_position = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    // <esc> will set this to true
    while !window.should_close() {
        raylib::begin_drawing();

        raylib::clear_background(Color::BLACK);

        raylib::begin_mode_3d(&mut camera);

        raylib::draw_cube(cube_position, 2.0, 2.0, 2.0, Color::RED);

        raylib::draw_cube_wires(cube_position, 2.0, 2.0, 2.0, Color::GREEN);

        raylib::draw_grid(GRID_SIZE, 1.0);

        raylib::end_mode_3d();

        raylib::end_drawing();
    }

    raylib::close_window();
}

// Local Variables:
// jinx-local-words: "esc"
// End:
