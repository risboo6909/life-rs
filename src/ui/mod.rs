extern crate opengl_graphics;
extern crate piston_window;

mod window;

use self::window::{WindowBase, PostAction, States};
use self::window::board::GameBoard;
use self::window::hud::HUDWindow;
use self::window::confirm::{ConfirmationWindow, UserChoice};

pub use super::structs::{GraphicsWindow, CellProp};

use super::{Resources, OPENGL};
use opengl_graphics::GlGraphics;
use engine::Engine;

use std::rc::Rc;
use std::cell::RefCell;

use piston_window::{Event, Input, Button, Key, Context, clear};

pub struct UI<'a> {

    cur_state: States,

    stack: Vec<Box<WindowBase + 'a>>,

    window: Rc<GraphicsWindow>,
    engine: Rc<RefCell<Engine<'a>>>,
    resources: Rc<RefCell<Resources>>,
}

impl<'a> UI<'a> {

    pub fn push(&mut self, w: Box<WindowBase + 'a>) {
        self.stack.push(w);
    }

    pub fn push_front(&mut self, w: Box<WindowBase + 'a>) {
        self.stack.insert(0, w);
    }

    pub fn get_window(&self) -> Rc<GraphicsWindow> {
        self.window.clone()
    }

    pub fn get_engine(&self) -> Rc<RefCell<Engine<'a>>> {
        self.engine.clone()
    }

    pub fn get_resources(&self) -> Rc<RefCell<Resources>> {
        self.resources.clone()
    }

    fn manage_windows(&mut self, e: &Event) {

        let mut to_remove = Vec::new();

        // update all windows one by one in order
        for (idx, window) in self.stack.iter_mut().enumerate() {

            let post_action = window.event_dispatcher(&e, &mut self.cur_state);

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

                                   self.cur_state = States::Paused;

                                    let confirm_window = Box::new(ConfirmationWindow::new(
                                        self.get_resources(), self.get_engine(),

                                            |engine, user_choice, cur_state| {
                                                if user_choice == UserChoice::Ok {
                                                    engine.borrow_mut().reset();
                                                } else if user_choice == UserChoice::Cancel {
                                                    *cur_state = States::Working;
                                                }
                                            }

                                    ));

                                    self.push_front(confirm_window);

                                }

                                _ => {

                                    self.manage_windows(&e);

                                }
                            }

                        }

                    }

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
        for window in &mut self.stack {
            window.paint(c, g)
        }

    }

}

pub fn new<'a>(window: Rc<GraphicsWindow>, engine: Rc<RefCell<Engine<'a>>>, resources: Rc<RefCell<Resources>>) -> UI<'a> {

    let mut ui = UI {
                      cur_state: States::Paused,

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
