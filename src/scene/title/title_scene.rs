use ggez::{Context, GameResult};

use crate::{asset::audio::Bgm, Asset};

#[derive(new)]
pub struct TitleState {
    render_state: TitleRenderState,
}

pub struct TitleRenderState;

pub fn init(ctx: &mut Context, asset: &mut Asset) -> GameResult<TitleState> {
    asset.audio.play_bgm(ctx, Bgm::Title)?;

    Ok(TitleState::new(TitleRenderState {}))
}

pub fn update(_: &mut Context, _: &mut TitleState) {}

pub fn render(_: &mut Context, _: &TitleState) {}
