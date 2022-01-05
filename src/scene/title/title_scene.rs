use ggez::{Context, GameResult};

use crate::{
    Asset,
    asset::{
        audio::Bgm,
    },
};

pub struct TitleState {
    render_state: TitleRenderState,
}

struct TitleRenderState;

fn init(ctx: &mut Context, asset: &mut Asset, _: &TitleState) -> GameResult {
    asset.audio.play_bgm(ctx, Bgm::Title)?;

    Ok(())
}

fn update(_: &mut Context, _: &mut TitleState) {


}

fn render(_: &mut Context, _: &TitleState) {}
