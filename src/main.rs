extern crate piston_window;

use piston_window::*;

mod board;


enum State {
    Working,
    Paused,
    Help,
}

fn main() {

    let mut window: PistonWindow = WindowSettings::new(
        "My Rust Life",
        [600, 600]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
        });
    }

}
