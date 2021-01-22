extern crate ncurses;

use ncurses::{endwin, getch, refresh};

use crate::graphics::cui::init_ncurses;
use crate::scene::{Title, TitleItem};
use crate::scene::title::cui::CuiTitle;

mod scene;
mod graphics;

fn main() {
    if init_ncurses().is_none() {
        eprintln!("Failed to initialize");
        return;
    }

    let title: CuiTitle = Title::new();

    loop {
        title.render();
        refresh();
        if getch() == (' ' as i32) { break; }
    };

    endwin();
}