use ggez::{Context, GameResult};

use crate::{asset::audio::Bgm, Asset, Next, SceneState};

#[derive(new)]
pub struct TitleState {
    draw_state: TitleDrawState,
}

pub struct TitleDrawState;

pub fn init(ctx: &mut Context, asset: &mut Asset) -> GameResult<TitleState> {
    asset.audio.play_bgm(ctx, Bgm::Title)?;

    Ok(TitleState::new(TitleDrawState {}))
}

pub fn update(_: &mut Context, state: TitleState) -> GameResult<Next> {
    Ok(Next::do_continue(state.into()))
}

pub fn draw(_: &mut Context, _: &TitleState, asset: &mut Asset) -> GameResult {
    Ok(())
}
