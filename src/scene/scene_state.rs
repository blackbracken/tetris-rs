use crate::scene::{fortyline::fortyline_scene::FortyLineState, title::title_scene::TitleState};

pub enum SceneState {
    ForTitle { state: TitleState },
    ForFortyLine { state: FortyLineState },
}
