use ggez::{Context, GameResult};

use crate::{
    asset::Asset,
    scenes,
    scenes::{
        play40line::Play40LineState,
        router::{
            Next::{Continue, Transit},
            SceneState::{ForPlay40Line, ForTitle},
        },
        title::TitleState,
    },
};

pub enum SceneState {
    ForTitle { state: TitleState },
    ForPlay40Line { state: Play40LineState },
}

impl Into<SceneState> for TitleState {
    fn into(self) -> SceneState {
        ForTitle { state: self }
    }
}

impl Into<SceneState> for Play40LineState {
    fn into(self) -> SceneState {
        ForPlay40Line { state: self }
    }
}

pub enum Ticket {
    ShowTitle,
    Play40Line,
}

impl Ticket {
    pub fn go(&self, ctx: &mut Context, asset: &mut Asset) -> GameResult<SceneState> {
        match &self {
            Ticket::ShowTitle => {
                scenes::title::init(ctx, asset)?;
                TitleState::new(ctx, asset).map(|state| ForTitle { state })
            }
            Ticket::Play40Line => {
                scenes::play40line::init(ctx, asset);
                Play40LineState::new(ctx).map(|state| ForPlay40Line { state })
            }
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
