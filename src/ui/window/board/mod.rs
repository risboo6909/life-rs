use super::{InactiveWindow, WindowBase};

use super::super::super::engine::Engine;
use super::super::super::board::{Board, CellDesc};
use super::super::super::structs::{CellProp, GameWindow};


pub struct GameBoard<'a> {

    window: GameWindow,
    engine: Engine<'a>,

}

impl<'a> WindowBase for GameBoard<'a> {

    fn paint(&self) {

    }

}

impl<'a> InactiveWindow for GameBoard<'a> {

}

pub fn new<'a>(window: GameWindow, engine: Engine<'a>) -> GameBoard<'a> {

    GameBoard {
        window: window,
        engine: engine
    }

}
