use crate::{scene::title::title_scene::TitleState, SceneState::ForTitle};

pub enum SceneState {
    ForTitle { state: TitleState },
}

impl Into<SceneState> for TitleState {
    fn into(self) -> SceneState {
        ForTitle { state: self }
    }
}
