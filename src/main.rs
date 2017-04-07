/// The Game of Life is my first experimental Rust project
/// to learn base features of the language.

extern crate piston_window;
extern crate opengl_graphics;
extern crate find_folder;

mod board;
mod engine;
mod cam;
mod structs;
mod ui;

use ui::new_board_window;

use structs::GameWindow;
use std::rc::Rc;

use find_folder::Search;
use piston_window::{OpenGL, Context, text, clear, rectangle, line,
                    Transformed, Event, Button, Input,
                    MouseButton, Key, PistonWindow, WindowSettings, Motion};

use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;

use engine::Engine;

const OPENGL: piston_window::OpenGL = OpenGL::V3_2;


pub struct Resources {
    font: GlyphCache<'static>
}

struct Game {
    ui_manager: ui::UI,
}

impl Game {

    fn new(width: f64, height: f64) -> Game {

        let window: PistonWindow = WindowSettings::new(
            "My Rust Life",
            [width as u32, height as u32]
        ).opengl(OPENGL)
            .samples(8)
            .exit_on_esc(true)
            .build()
            .unwrap();

        Game {
            ui_manager: ui::new(Rc::new(GameWindow::new(width, height, window)),
                                Resources {
                                    font: GlyphCache::new(Search::ParentsThenKids(3, 3).
                                    for_folder("assets").unwrap().
                                    join("Roboto-Regular.ttf")).unwrap() }),
        }
    }

    fn event_dispatcher(&mut self) {
        self.ui_manager.event_dispatcher();
    }

}


fn main() {
    let mut game = Game::new(1024.0, 768.0);

    game.event_dispatcher();
}
