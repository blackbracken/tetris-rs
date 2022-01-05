use crate::scene::scene_state::SceneState::ForTitle;
use crate::{
    scene::{
        scene_state::SceneState,
        title::{title_scene, title_scene::TitleState},
    },
    Asset,
};
use ggez::{Context, GameResult};

pub enum Ticket {
    ShowTitle,
}

impl Ticket {
    pub fn go(&self, ctx: &mut Context, asset: &mut Asset) -> GameResult<SceneState> {
        match &self {
            Ticket::ShowTitle => title_scene::init(ctx, asset).map(|state| ForTitle { state }),
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
        Next::Continue { state }
    }

    pub fn transit(ticket: Ticket) -> Next {
        Next::Transit { ticket }
    }
}
