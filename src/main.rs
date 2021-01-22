extern crate ncurses;

use ncurses::{endwin, getch, refresh};

use crate::graphics::cui::init_ncurses;
use crate::scene::{Destination, InputAction, Title, TitleItem};
use crate::scene::title::cui::CuiTitle;

mod scene;
mod graphics;

fn main() {
    if init_ncurses().is_none() {
        eprintln!("Failed to initialize");
        return;
    }

    let mut title: CuiTitle = Title::new();

    loop {
        title.render();
        refresh();

        match title.wait_input() {
            InputAction::Go(dest) => match dest {
                Destination::Title => {}
                Destination::Exit => break
            }
            InputAction::Nothing => {}
        }
    };

    endwin();
}