use ggez::{Context, GameResult};

use super::scene_state::SceneState;

pub enum Ticket {
    ShowTitle,
    Play40Line,
}

impl Ticket {
    pub fn go(&self, ctx: &mut Context) -> GameResult<SceneState> {
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
