use std::convert::{TryFrom, TryInto};

use ncurses::{attrset, getch, KEY_DOWN, KEY_UP, mvaddstr, stdscr};

use crate::graphics::color::{CuiColor, ViewColor};
use crate::graphics::cui::{draw_frame, END_POINT, START_POINT};
use crate::scene::{Destination, InputAction, Title, TitleItem};
use crate::scene::cui::HasTextForCui;

const TITLE_AA: [&str; 6] = [
    " _       _                         ",
    "| |     | |      (_)               ",
    "| |_ ___| |_ _ __ _ ___   _ __ ___ ",
    "| __/ _ \\ __| '__| / __| | '__/ __|",
    "| ||  __/ |_| |  | \\__ \\_| |  \\__ \\",
    " \\__\\___|\\__|_|  |_|___(_)_|  |___/",
];

pub struct CuiTitle {
    is_initialized: bool,
    selected: TitleItem,
}

impl Title for CuiTitle {
    fn new() -> Self {
        CuiTitle { is_initialized: false, selected: TitleItem::Start40line }
    }

    fn render(&self) {
        if !self.is_initialized {
            draw_frame(stdscr(), ViewColor::WindowFrame, START_POINT, END_POINT);

            attrset(ViewColor::WindowFrame.to_attr());
            for (idx, line) in TITLE_AA.iter().enumerate() {
                mvaddstr((idx as i32) + 3, END_POINT.x / 2 - (TITLE_AA[0].len() as i32) / 2, line);
            }
        }

        TitleItem::all()
            .iter()
            .map(|item| {
                let selector = if &self.selected == item { "> " } else { "  " };
                selector.to_owned() + item.convert_into_text().as_str()
            })
            .enumerate()
            .for_each(|(idx, item_text)| {
                mvaddstr(15 + 2 * (idx as i32), END_POINT.x / 2 - (item_text.len() as i32) / 2 - 2, item_text.as_str());
            });
    }

    fn handle_input(&mut self) -> InputAction {
        let key_space = ' ' as i32;
        let key_w = 'w' as i32;
        let key_s = 's' as i32;

        // TODO: write if-arms smarter
        let input = getch();
        if key_space == input {
            // TODO: implement
            InputAction::Go(Destination::Exit)
        } else if KEY_UP == input || key_w == input {
            self.selected.prev().map(|prev| { self.selected = prev });
            InputAction::Nothing
        } else if KEY_DOWN == input || key_s == input {
            self.selected.next().map(|next| { self.selected = next });
            InputAction::Nothing
        } else {
            InputAction::Nothing
        }
    }

    fn go_up(&self) -> Self {
        unimplemented!();
    }

    fn go_down(&self) -> Self {
        unimplemented!()
    }

    fn select(&self) -> Option<Destination> {
        unimplemented!()
    }
}

impl HasTextForCui for TitleItem {
    fn convert_into_text(&self) -> String {
        use TitleItem::*;
        String::from(match self {
            Start40line => "Play 40LINE",
            Exit => "Exit"
        })
    }
}