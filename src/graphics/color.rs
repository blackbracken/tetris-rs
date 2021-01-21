use ncurses::{attr_t, COLOR_CYAN, COLOR_PAIR, COLOR_RED, COLOR_WHITE, init_pair};

pub trait CuiColor {
    fn to_code(&self) -> i16;
    fn to_attr(&self) -> attr_t { COLOR_PAIR(self.to_code()) }
}

pub enum ViewColor {
    Air,
    WindowFrame,
}

impl CuiColor for ViewColor {
    fn to_code(&self) -> i16 {
        match self {
            ViewColor::Air => 0,
            ViewColor::WindowFrame => 1,
        }
    }
}

pub fn prepare_for_cui() {
    init_pair(ViewColor::Air.to_code(), -1, -1);
    init_pair(ViewColor::WindowFrame.to_code(), COLOR_CYAN, -1);
}