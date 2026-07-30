

mod osk_graphics;
use osk_graphics::{poll_events, Point};


fn main() {
    let width: u32 = 600;
    let height: u32 = 600; 

    let win: osk_graphics::Window = osk_graphics::Window::open(width, height, "Oskar's Graphics Window").expect("Failed to open window");

    win.set_background(osk_graphics::Colour::BLACK);
    while poll_events() {
        win.begin_frame();

        win.draw_triangle(Point {x: 150.0, y: 100.0}, Point {x: 250.0, y: 400.0}, Point {x: 50.0, y: 400.0}, osk_graphics::Colour::RED);

        win.end_frame();
    }
}
