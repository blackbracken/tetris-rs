use crate::graphics::color::ViewColor;
use crate::graphics::cui::{draw_frame, END_POINT, START_POINT};
use crate::scene::TitleItem::{Exit, Start40line};

pub mod title {
    pub mod cui;
}

pub mod cui;

pub enum InputAction {
    Go(Destination),
    Nothing,
}

pub trait Title {
    fn new() -> Self;
    fn render(&self);
    fn handle_input(&mut self) -> InputAction;

    fn go_up(&self) -> Self;
    fn go_down(&self) -> Self;
    fn select(&self) -> Option<Destination>;
}

#[derive(PartialEq)]
pub enum TitleItem {
    Start40line,
    Exit,
}

impl TitleItem {
    fn all() -> [TitleItem; 2] {
        [Start40line, Exit] // TODO: implement smarter
    }

    fn next(&self) -> Option<TitleItem> {
        match self {
            Start40line => Some(Exit),
            Exit => None,
        }
    }

    fn prev(&self) -> Option<TitleItem> {
        match self {
            Start40line => None,
            Exit => Some(Start40line),
        }
    }
}

// TODO: consider this should be implemented here
pub enum Destination {
    Title,
    Exit,
}