use ggez::Context;

use crate::router::Next::{Continue, Transit};
use crate::router::ViewState::ForTitle;
use crate::view::title::TitleState;

pub enum ViewState {
    ForTitle { state: TitleState }
}

pub enum Ticket {
    ShowTitle,
    Play40Line,
}

impl Ticket {
    pub fn go(&self, ctx: &mut Context) -> ViewState {
        match &self {
            Ticket::ShowTitle => ForTitle { state: TitleState::new(ctx).unwrap() }, // TODO: error handling
            Ticket::Play40Line => unimplemented!(),
        }
    }
}

pub enum Next {
    Continue { state: ViewState },
    Transit { ticket: Ticket },
    Exit,
}

impl Next {
    pub fn do_continue(state: ViewState) -> Next {
        Continue { state }
    }

    pub fn transit(ticket: Ticket) -> Next {
        Transit { ticket }
    }
}