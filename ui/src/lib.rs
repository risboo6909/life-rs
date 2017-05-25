extern crate opengl_graphics;
extern crate piston_window;
extern crate engine;

mod windows;
mod cam;

use cam::Cam;

use self::windows::{WindowBase, PostAction, States};
use self::windows::board::GameBoard;
use self::windows::hud::HUDWindow;
use self::windows::confirm::{ConfirmationWindow, UserChoice};
use self::windows::info::InfoWindow;

use engine::Engine;

use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;

use std::rc::Rc;
use std::cell::{RefCell, Cell};

use piston_window::{PistonWindow, OpenGL, Event, Input, Button, Key, Context, clear};

pub const OPENGL: piston_window::OpenGL = OpenGL::V3_2;


pub struct UI<'a> {

    cur_state: Cell<States>,

    stack: Vec<Box<WindowBase + 'a>>,

    window: Rc<GraphicsWindow>,
    engine: Rc<RefCell<Engine>>,
    resources: Rc<RefCell<Resources>>,
}

pub struct Resources {
    pub font: GlyphCache<'static>
}

pub struct GraphicsWindow {
    window: Rc<RefCell<PistonWindow>>,

    width: f64,
    height: f64,
}

pub struct CellProp {
    cell_width: f64,
    cell_height: f64,
}

impl CellProp {
    pub fn new(cell_width: f64, cell_height: f64) -> Self {
        CellProp { cell_width: cell_width, cell_height: cell_height }
    }

    #[inline]
    pub fn get_width(&self, cam: &Cam) -> f64 {
        self.cell_width * cam.get_scale()
    }

    #[inline]
    pub fn get_height(&self, cam: &Cam) -> f64 {
        self.cell_height * cam.get_scale()
    }

    #[inline]
    pub fn get_half_width(&self, cam: &Cam) -> f64 {
        0.5 * self.get_width(&cam)
    }

    #[inline]
    pub fn get_half_height(&self, cam: &Cam) -> f64 {
        0.5 * self.get_height(&cam)
    }
}


impl GraphicsWindow {

    pub fn new(window_width: f64, window_height: f64, window: PistonWindow) -> Self {
        GraphicsWindow { width: window_width,
                         height: window_height,
                         window: Rc::new(RefCell::new(window)) }
    }

    #[inline]
    pub fn get_width(&self) -> f64 {
        self.width
    }

    #[inline]
    pub fn get_height(&self) -> f64 {
        self.height
    }

    #[inline]
    pub fn get_half_width(&self) -> f64 {
        0.5 * self.get_width()
    }

    #[inline]
    pub fn get_half_height(&self) -> f64 {
        0.5 * self.get_height()
    }

    #[inline]
    pub fn get_window(&self) -> &Rc<RefCell<PistonWindow>> {
        &self.window
    }

}


impl<'a> UI<'a> {

    pub fn push(&mut self, w: Box<WindowBase + 'a>) {
        self.stack.push(w);
    }

    pub fn push_front(&mut self, w: Box<WindowBase + 'a>) {
        if self.stack.len() != 0 {
            if !self.stack[0].is_modal() {
                self.stack.insert(0, w);
            }
        }
    }

    pub fn get_window(&self) -> Rc<GraphicsWindow> {
        self.window.clone()
    }

    pub fn get_engine(&self) -> Rc<RefCell<Engine>> {
        self.engine.clone()
    }

    pub fn get_resources(&self) -> Rc<RefCell<Resources>> {
        self.resources.clone()
    }

    fn create_prompt_window<F: 'a>(&mut self, msg: &'a str, callback: F)  where
        F: FnMut(Rc<RefCell<Engine>>, UserChoice) {

        let confirm_window = Box::new(ConfirmationWindow::new(self.get_resources(), self.get_engine(),
                                         callback, msg,
                                         self.get_window().get_width(),
                                         self.get_window().get_height()));

        self.push_front(confirm_window);
    }

    fn create_info_window(&mut self, msg: &'a str) {

        let info_window = Box::new(InfoWindow::new(
            self.get_resources(), self.get_engine(),
            msg,
            self.get_window().get_width(),
            self.get_window().get_height()
        ));

        self.push_front(info_window);

    }

    fn manage_windows(&mut self, e: &Event) {

        let mut to_remove = Vec::new();

        // update all windows one by one in order
        for (idx, window) in self.stack.iter_mut().enumerate() {

            let post_action = window.event_dispatcher(&e, &self.cur_state);

            match post_action {

                PostAction::Transfer => {},
                PostAction::Stop => break,
                PostAction::Pop => to_remove.push(idx),

            }
        }

        // remove windows that scheduled to be removed earlier
        for window_idx in to_remove {
            self.stack.remove(window_idx);
        }

    }

    pub fn event_dispatcher(&mut self) -> PostAction {

        let mut gl = GlGraphics::new(OPENGL);

        loop {

            let event = { self.window.get_window().borrow_mut().next() };

            match event {

                Some(e) => {

                    match e {

                        // paint all the windows first
                        Event::Render(args) => {
                            gl.draw(args.viewport(), |c, g| self.paint_all(c, g));
                        }

                        // process other events
                        ref some_event => {

                            match some_event {

                                &Event::Input(Input::Press(Button::Keyboard(Key::C))) => {

                                    // clear board and reset counters

                                    self.cur_state.set(States::Paused);

                                    self.create_prompt_window(
                                        "Are you sure you want to clear the board?",
                                        |engine, user_choice| {
                                            if user_choice == UserChoice::Ok {
                                                engine.borrow_mut().reset();
                                            }
                                        }
                                    );
                                }

                                &Event::Input(Input::Press(Button::Keyboard(Key::P))) => {
                                    // pause/unpause
                                    if self.cur_state.get() == States::Working {
                                        self.cur_state.set(States::Paused);
                                    } else {
                                        self.cur_state.set(States::Working);
                                    }
                                }

                                &Event::Input(Input::Press(Button::Keyboard(Key::S))) => {
                                    // enter step by step mode
                                    if self.cur_state.get() == States::Working || self.cur_state.get() == States::Paused {
                                        self.cur_state.set(States::StepByStep);
                                    }
                                }

                                &Event::Input(Input::Press(Button::Keyboard(Key::R))) => {
                                    if self.cur_state.get() == States::Paused {

                                        let engine = self.get_engine();

                                        if engine.borrow().get_board().is_infinite() {
                                            self.create_info_window("Can't generate random \
                                            configuration for infinite board");
                                        } else {
                                            self.create_prompt_window(
                                                "Current position will be lost, ok?",
                                                |engine, user_choice| {
                                                    if user_choice == UserChoice::Ok {
                                                        // generate random board
                                                        let board = engine.borrow().create_random(0.3);
                                                        engine.borrow_mut().set_board(board);
                                                    }
                                                }
                                            );
                                        }

                                    }
                                }

                                // do nothing if nothing matched
                                _ => {}

                            }

                        }

                    }

                    self.manage_windows(&e);

                }

                None => break
            }
        }

        PostAction::Transfer

    }

    pub fn paint_all(&mut self, c: Context, g: &mut GlGraphics) {

        // clear background
        clear([0.0, 0.0, 0.0, 1.0], g);

        // and paint all windows one by one in order
        for window in &mut self.stack.iter_mut().rev() {
            window.paint(c, g)
        }

    }

}

pub fn new<'a>(window: Rc<GraphicsWindow>, engine: Rc<RefCell<Engine>>,
               resources: Rc<RefCell<Resources>>) -> UI<'a> {

    let mut ui = UI {
                      cur_state: Cell::new(States::Paused),

                      stack: Vec::new(),
                      window: window,
                      engine: engine,
                      resources: resources,
                    };

    let board_window = Box::new(GameBoard::new(ui.get_window(),
                                               ui.get_engine()));

    let hud_window = Box::new(HUDWindow::new(ui.get_resources(),
                                             ui.get_engine()));

    ui.push(board_window);
    ui.push(hud_window);

    ui
}
