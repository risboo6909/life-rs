pub mod confirm;
pub mod board;

pub trait WindowBase {

    fn paint(&self);

}

pub trait ActiveWindow: WindowBase {

    fn event_dispatcher(&self);

}


pub trait InactiveWindow: WindowBase {


}
