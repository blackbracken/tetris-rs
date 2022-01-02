use ggez::Context;

use crate::scenes::scene_state::SceneState;

struct FortyLineState;
impl SceneState<!> for FortyLineState {
    fn game_state(&self) -> &! {
        todo!()
    }

    fn is_paused(&self) -> bool {
        false
    }
}

fn init(_: &mut Context, _: &FortyLineState) {}

fn update(_: &mut Context, _: &mut FortyLineState) {}

fn render(_: &mut Context, _: &FortyLineState) {}
