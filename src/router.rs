use ggez::{Context, GameResult};

use crate::resource::SharedResource;
use crate::router::Next::{Continue, Transit};
use crate::router::ViewState::{ForPlay40Line, ForTitle};
use crate::view::play40line::Play40LineState;
use crate::view::title::TitleState;

pub enum ViewState {
    ForTitle { state: TitleState },
    ForPlay40Line { state: Play40LineState },
}

pub enum Ticket {
    ShowTitle,
    Play40Line,
}

impl Ticket {
    pub fn go(&self, ctx: &mut Context, resource: &SharedResource) -> GameResult<ViewState> {
        match &self {
            Ticket::ShowTitle => TitleState::new(ctx, resource).map(|state| ForTitle { state }),
            Ticket::Play40Line => Play40LineState::new(ctx).map(|state| ForPlay40Line { state }),
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