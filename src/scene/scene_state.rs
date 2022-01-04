use crate::scene::fortyline::fortyline_scene::FortyLineState;

pub enum SceneState {
    ForFortyLine { render_state: FortyLineState },
}
