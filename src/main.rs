use raylib::raylib;
use raylib::Color;
use raylib::Window;

const SCREEN_WIDTH: i32 = 600;
const SCREEN_HEIGHT: i32 = 600;
const GRID_SIZE: i32 = 40;

fn main() {
    let window = Window::new(SCREEN_WIDTH, SCREEN_HEIGHT, "hello world");

    // <esc> will set this to true
    while !window.should_close() {
        raylib::begin_drawing();

        raylib::clear_background(Color::BLACK);

        raylib::draw_rectangle(0, 0, 50, 100, Color::RED);

        raylib::end_drawing();
    }

    raylib::close_window();
}

// Local Variables:
// jinx-local-words: "esc"
// End:
