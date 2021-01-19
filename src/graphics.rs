extern crate ncurses;

use ncurses::{attr_t, COLOR_PAIR, COLOR_RED, COLOR_WHITE, init_pair};

pub(crate) trait TetrisColor {
    fn to_code(&self) -> i16;
    fn to_attr(&self) -> attr_t { COLOR_PAIR(self.to_code()) }
}

pub(crate) enum ViewColor {
    Air,
    WindowFrame,
    SomeColor,
}

pub(crate) struct Pos {
    pub x: i32,
    pub y: i32,
}

impl TetrisColor for ViewColor {
    fn to_code(&self) -> i16 {
        match self {
            ViewColor::Air => 0,
            ViewColor::WindowFrame => 1,
            ViewColor::SomeColor => 2,
        }
    }
}

pub(crate) fn prepare_colors() {
    init_pair(ViewColor::Air.to_code(), -1, -1);
    init_pair(ViewColor::WindowFrame.to_code(), COLOR_WHITE, -1);
    init_pair(ViewColor::SomeColor.to_code(), COLOR_RED, -1);
}