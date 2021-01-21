extern crate ncurses;

use ncurses::{endwin, getch, refresh};

use crate::graphics::cui::init_ncurses;
use crate::scene::scene::Title;
use crate::scene::title::CuiTitle;

mod scene;
mod graphics;

fn main() {
    if let None = init_ncurses() {
        eprintln!("Failed to initialize");
        return;
    }

    let title: CuiTitle = Title::new();

    loop {
        title.render();
        refresh();
        getch();
    };

    endwin();
}