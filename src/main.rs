extern crate ncurses;

use ncurses::{addstr, attrset, COLOR_PAIR, curs_set, endwin, getch, initscr, keypad, mvaddstr, noecho, refresh, start_color, stdscr, use_default_colors};
use ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE;

use crate::graphics::{prepare_colors, TetrisColor, ViewColor};

mod graphics;

fn main() {
    if let None = init_ncurses() {
        eprintln!("Failed to initialize");
        return;
    }

    attrset(ViewColor::WindowFrame.to_attr());
    mvaddstr(3, 30, "Hello, world!");

    refresh();
    getch();

    endwin();
}

fn init_ncurses() -> Option<()> {
    initscr();

    curs_set(CURSOR_INVISIBLE)?;

    noecho();
    keypad(stdscr(), true);
    start_color();
    use_default_colors();

    prepare_colors();

    Some(())
}
