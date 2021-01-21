use ncurses::stdscr;

use crate::graphics::color::ViewColor;
use crate::graphics::cui::{draw_frame, END_POINT, START_POINT};
use crate::scene::scene::{Destination, Title};

pub struct CuiTitle {
    is_initialized: bool,
}

impl Title for CuiTitle {
    fn new() -> Self {
        CuiTitle { is_initialized: false }
    }

    fn render(&self) {
        if !self.is_initialized {
            draw_frame(stdscr(), ViewColor::WindowFrame, START_POINT, END_POINT);
        }
    }

    fn go_up(&self) -> Self {
        unimplemented!()
    }

    fn go_down(&self) -> Self {
        unimplemented!()
    }

    fn select(&self) -> Option<Destination> {
        unimplemented!()
    }
}