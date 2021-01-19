extern crate ncurses;

use ncurses::{attr_off, attr_on, attrset, curs_set, endwin, getch, initscr, keypad, mvaddstr, mvwaddch, noecho, refresh, start_color, stdscr, use_default_colors, WINDOW};
use ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE;

use crate::graphics::{Pos, prepare_colors, TetrisColor, ViewColor};

mod graphics;

fn main() {
    if let None = init_ncurses() {
        eprintln!("Failed to initialize");
        return;
    }

    attrset(ViewColor::WindowFrame.to_attr());
    mvaddstr(3, 30, "Hello, world!");

    draw_frame(stdscr(), ViewColor::SomeColor, Pos { x: 0, y: 0 }, Pos { x: 30, y: 5 });

    refresh();
    getch();

    endwin();
}

fn draw_frame<C: TetrisColor>(window: WINDOW, color: C, start: Pos, end: Pos) {
    attrset(color.to_attr());

    (start.x..=end.x).for_each(|x| {
        mvwaddch(window, start.y, x, '=' as u32);
        mvwaddch(window, end.y, x, '=' as u32);
    });
    ((start.y + 1)..end.y).for_each(|y| {
        mvwaddch(window, y, start.x, 'I' as u32);
        mvwaddch(window, y, end.x, 'I' as u32);
    });
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
