use ggez::{Context, GameResult};

use crate::asset::Asset;
use crate::router::Next::{Continue, Transit};
use crate::router::SceneState::{ForPlay40Line, ForTitle};
use crate::scene::play40line::Play40LineState;
use crate::scene::title::TitleState;

pub enum SceneState {
    ForTitle { state: TitleState },
    ForPlay40Line { state: Play40LineState },
}

pub enum Ticket {
    ShowTitle,
    Play40Line,
}

impl Ticket {
    pub fn go(&self, ctx: &mut Context, asset: &mut Asset) -> GameResult<SceneState> {
        match &self {
            Ticket::ShowTitle => TitleState::new(ctx, asset).map(|state| ForTitle { state }),
            Ticket::Play40Line => Play40LineState::new(ctx).map(|state| ForPlay40Line { state }),
        }
    }
}

pub enum Next {
    Continue { state: SceneState },
    Transit { ticket: Ticket },
    Exit,
}

impl Next {
    pub fn do_continue(state: SceneState) -> Next {
        Continue { state }
    }

    pub fn transit(ticket: Ticket) -> Next {
        Transit { ticket }
    }
}