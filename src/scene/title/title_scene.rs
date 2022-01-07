use ggez::{
    graphics,
    graphics::{DrawParam, Text, TextFragment},
    Context, GameResult,
};
use indoc::indoc;

use crate::{asset::audio::Bgm, Asset, Next, SceneState, WINDOW_WIDTH};

static TITLE_ASCII: &str = indoc!(
    r"
 __           __
/\ \__       /\ \__         __
\ \ ,_\    __\ \ ,_\  _ __ /\_\    ____           _ __   ____
 \ \ \/  /'__`\ \ \/ /\`'__\/\ \  /',__\  _______/\`'__\/',__\
  \ \ \_/\  __/\ \ \_\ \ \/ \ \ \/\__, `\/\______\ \ \//\__, `\
   \ \__\ \____\\ \__\\ \_\  \ \_\/\____/\/______/\ \_\\/\____/
    \/__/\/____/ \/__/ \/_/   \/_/\/___/           \/_/ \/___/
    "
);

#[derive(new)]
pub struct TitleState {
    draw_state: TitleDrawState,
}

pub struct TitleDrawState {
    title_texts_ascii: Vec<Text>,
}

impl TitleDrawState {
    fn new() -> TitleDrawState {
        let title_texts_ascii = TITLE_ASCII
            .split("\n")
            .into_iter()
            .map(|line| Text::new(TextFragment::from(line)))
            .collect();

        TitleDrawState { title_texts_ascii }
    }
}

pub fn init(ctx: &mut Context, asset: &mut Asset) -> GameResult<TitleState> {
    asset.audio.play_bgm(ctx, Bgm::Title)?;

    Ok(TitleState::new(TitleDrawState::new()))
}

pub fn update(_: &mut Context, state: TitleState) -> GameResult<Next> {
    Ok(Next::do_continue(state.into()))
}

pub fn draw(ctx: &mut Context, state: &TitleState, asset: &mut Asset) -> GameResult {
    let draw_state = &state.draw_state;

    graphics::clear(ctx, asset.color.background);

    let title_max_width = draw_state.title_texts_ascii.get(4).unwrap().width(ctx);
    for (idx, text) in draw_state.title_texts_ascii.iter().enumerate() {
        let x = WINDOW_WIDTH / 2. - title_max_width / 2.;
        let y = 50. + (15 * idx) as f32;

        graphics::draw(ctx, text, DrawParam::default().dest([x, y]))?;
    }

    graphics::present(ctx)?;

    Ok(())
}
