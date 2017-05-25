/// The Game of Life is my first experimental Rust project
/// to learn base features of the language.

extern crate piston_window;
extern crate opengl_graphics;
extern crate find_folder;
extern crate engine;
extern crate ui;

use ui::GraphicsWindow;
use std::rc::Rc;
use std::cell::RefCell;

use find_folder::Search;
use piston_window::{PistonWindow, WindowSettings};

use opengl_graphics::glyph_cache::GlyphCache;

use engine::engine::Engine;


struct Game<'a> {
    ui_manager: ui::UI<'a>,
}

impl<'a> Game<'a> {

    fn new(width: f64, height: f64) -> Game<'a> {

        let window: PistonWindow = WindowSettings::new(
            "My Rust Life",
            [width as u32, height as u32]
        ).opengl(ui::OPENGL)
            .samples(8)
            .exit_on_esc(true)
            .build()
            .unwrap();

        Game {
            ui_manager: ui::new(Rc::new(GraphicsWindow::new(width, height, window)),
                                Rc::new(RefCell::new(Engine::new(Some(200), Some(200)))),
                                Rc::new(RefCell::new(ui::Resources {
                                    font: GlyphCache::new(Search::ParentsThenKids(3, 3).
                                    for_folder("assets").unwrap().
                                    join("Roboto-Regular.ttf")).unwrap()
                                }))
            ),
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
