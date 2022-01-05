use crate::scene::{fortyline::fortyline_scene::FortyLineState, title::title_scene::TitleState};
use crate::SceneState::ForTitle;

pub enum SceneState {
    ForTitle { state: TitleState },
}

impl Into<SceneState> for TitleState {
    fn into(self) -> SceneState {
        ForTitle { state: self }
    }
}
