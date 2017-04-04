// Simple confirmation window

use super::{ActiveWindow, WindowBase};

pub struct ConfirmationWindow {
    state: isize,
}

impl WindowBase for ConfirmationWindow {

    fn paint(&self) {

    }

}

impl ActiveWindow for ConfirmationWindow {

    fn event_dispatcher(&self) {

    }

}
