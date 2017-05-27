/// The Game of Life is my first experimental Rust project
/// to learn base features of the language.

extern crate clap;
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

use engine::Engine;

use clap::{App, Arg};


struct Game<'a> {
    ui_manager: ui::UI<'a>,
}

impl<'a> Game<'a> {

    fn new(screen_width: f64, screen_height: f64,
           board_cols: Option<u32>, board_rows: Option<u32>) -> Game<'a> {

        let window: PistonWindow = WindowSettings::new(
            "Conway's Game of Life",
            [screen_width as u32, screen_height as u32]
        ).opengl(ui::OPENGL)
            .samples(8)
            .exit_on_esc(true)
            .build()
            .unwrap();

        Game {
            ui_manager: ui::new(Rc::new(GraphicsWindow::new(screen_width, screen_height, window)),
                                Rc::new(RefCell::new(Engine::new(board_cols, board_rows))),
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

    let matches = App::new("Conway's Game of Life")
        .version("1.0")
        .author("Boris T. <ttyv00@gmail.com>")
        .arg(Arg::with_name("cols")
             .long("cols")
             .help("Sets a board width in cells, infinite if omitted")
             .value_name("COLS")
             .takes_value(true))
        .arg(Arg::with_name("rows")
             .long("rows")
             .help("Sets a board height in cells, infinite if omitted")
             .value_name("ROWS")
             .takes_value(true))
        .arg(Arg::with_name("width")
             .long("width")
             .help("Sets a screen width, default is 1024")
             .value_name("WIDTH")
             .takes_value(true))
        .arg(Arg::with_name("height")
             .long("height")
             .help("Sets a screen height, default is 768")
             .value_name("HEIGHT")
             .takes_value(true))

        .get_matches();

    let board_cols = matches.value_of("cols").map_or(None, |n| Some(n.parse::<u32>().unwrap()));
    let board_rows = matches.value_of("rows").map_or(None, |n| Some(n.parse::<u32>().unwrap()));
    let scr_width = matches.value_of("width").map_or(1024.0, |n| n.parse::<f64>().unwrap());
    let scr_height = matches.value_of("height").map_or(768.0, |n| n.parse::<f64>().unwrap());

    let mut game = Game::new(scr_width, scr_height, board_cols, board_rows);

    game.event_dispatcher();
}
