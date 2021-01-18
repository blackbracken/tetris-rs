extern crate ncurses;

use ncurses::{addstr, endwin, initscr, refresh, getch};

fn main() {
    initscr();

    addstr("hello, world!");

    refresh();

    getch();

    endwin();
}
