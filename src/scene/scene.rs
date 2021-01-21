use ncurses::stdscr;

use crate::graphics::color::ViewColor;
use crate::graphics::cui::{draw_frame, END_POINT, START_POINT};

pub trait Scene {
    fn start() -> Destination;
}

pub trait Title {
    fn new() -> Self;
    fn render(&self);

    fn go_up(&self) -> Self;
    fn go_down(&self) -> Self;
    fn select(&self) -> Option<Destination>;
}

// TODO: consider this should be implemented here
pub enum Destination {
    Title
}