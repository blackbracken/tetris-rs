use ncurses::{attrset, curs_set, initscr, keypad, mvwaddch, noecho, start_color, stdscr, use_default_colors, WINDOW};
use ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE;

use crate::graphics::color::{CuiColor, prepare_for_cui};
use crate::graphics::util::Pos;

pub const START_POINT: Pos = Pos { x: 0, y: 0 };
pub const END_POINT: Pos = Pos { x: 80, y: 40 };

pub fn init_ncurses() -> Option<()> {
    initscr();

    curs_set(CURSOR_INVISIBLE)?;

    noecho();
    keypad(stdscr(), true);
    start_color();
    use_default_colors();

    prepare_for_cui();

    Some(())
}

pub fn draw_frame<C: CuiColor>(window: WINDOW, color: C, start: Pos, end: Pos) {
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