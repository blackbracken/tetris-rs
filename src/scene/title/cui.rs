use ncurses::{attrset, mvaddstr, stdscr};

use crate::graphics::color::{CuiColor, ViewColor};
use crate::graphics::cui::{draw_frame, END_POINT, START_POINT};
use crate::scene::{Destination, Title, TitleItem};
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
                if &self.selected == item {
                    "> ".to_owned() + item.convert_into_text().as_str()
                } else {
                    item.convert_into_text()
                }
            })
            .enumerate()
            .for_each(|(idx, item_text)| {
                mvaddstr(15 + 2 * (idx as i32), 30, item_text.as_str());
            });
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