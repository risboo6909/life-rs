/// The Game of Life is my first experimental Rust project
/// to learn base features of the language.

#[macro_use]
extern crate clap;
extern crate piston_window;
extern crate opengl_graphics;
extern crate find_folder;
extern crate engine;
extern crate ui;

use std::rc::Rc;
use std::cell::RefCell;

use find_folder::Search;
use piston_window::{PistonWindow, WindowSettings};

use opengl_graphics::glyph_cache::GlyphCache;

use clap::{App, Arg};
use engine::loader::from_file as load_from_file;


struct Game<'a> {
    ui_manager: ui::UI<'a>,
}

impl<'a> Game<'a> {

    fn new(screen_width: f64, screen_height: f64,
           board_cols: Option<usize>, board_rows: Option<usize>) -> Game<'a> {

        let window: PistonWindow = WindowSettings::new(
            "Conway's Game of Life",
            [screen_width as u32, screen_height as u32]
        ).opengl(ui::OPENGL)
            .samples(8)
            .exit_on_esc(true)
            .build()
            .unwrap();

        Game {
            ui_manager: ui::new(Rc::new(ui::GraphicsWindow::new(screen_width, screen_height, window)),
                                Rc::new(RefCell::new(engine::Engine::new(None, board_cols, board_rows))),
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
             .help("Sets game window width, default is 1024")
             .value_name("WIDTH")
             .default_value("1024")
             .takes_value(true))
        .arg(Arg::with_name("height")
            .long("height")
            .help("Sets game window height, default is 768")
            .value_name("HEIGHT")
            .default_value("768")
            .takes_value(true))
        .arg(Arg::with_name("file")
            .long("file")
            .help("Read configuration from a file")
            .value_name("FILE")
            .takes_value(true))

        .get_matches();

    let board_cols = value_t!(matches, "cols", usize).ok();
    let board_rows = value_t!(matches, "rows", usize).ok();

    let scr_width = value_t_or_exit!(matches, "width", f64);
    let scr_height = value_t_or_exit!(matches, "height", f64);

    let file_name = value_t!(matches, "file", String).ok();
    load_from_file(file_name);

    let mut game = Game::new(scr_width, scr_height, board_cols, board_rows);

    game.event_dispatcher();
}
