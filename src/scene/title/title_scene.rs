use std::collections::HashMap;

use ggez::graphics::PxScale;
use ggez::{
    graphics,
    graphics::{DrawParam, Text, TextFragment},
    Context, GameResult,
};
use indoc::indoc;

use crate::scene::title::selected_item::SelectedItem;
use crate::{asset::audio::Bgm, Asset, Next, SceneState, WINDOW_HEIGHT, WINDOW_WIDTH};

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
    cursor: SelectedItem,
}

pub struct TitleDrawState {
    title_texts_ascii: Vec<Text>,
    selected_item_text_hash: HashMap<SelectedItem, Text>,
}

impl TitleDrawState {
    fn new() -> TitleDrawState {
        let title_texts_ascii = TITLE_ASCII
            .split("\n")
            .into_iter()
            .map(|line| Text::new(TextFragment::from(line)))
            .collect();

        let selected_item_text_hash = SelectedItem::all()
            .into_iter()
            .map(|item| {
                let name = item.name().to_owned();

                (
                    item,
                    Text::new(TextFragment::new(name).scale(PxScale::from(32.))),
                )
            })
            .collect();

        TitleDrawState {
            title_texts_ascii,
            selected_item_text_hash,
        }
    }
}

pub fn init(ctx: &mut Context, asset: &mut Asset) -> GameResult<TitleState> {
    asset.audio.play_bgm(ctx, Bgm::Title)?;

    Ok(TitleState {
        draw_state: TitleDrawState::new(),
        cursor: SelectedItem::PlayFortyLine,
    })
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

    for (idx, item) in SelectedItem::all().iter().enumerate() {
        let text = draw_state.selected_item_text_hash.get(item).unwrap();
        let x = WINDOW_WIDTH / 2. - text.width(ctx) / 2.;
        let y = WINDOW_HEIGHT / 3. + (50 * idx) as f32;

        graphics::draw(ctx, text, graphics::DrawParam::default().dest([x, y]))?;

        if item == &state.cursor {
            let cursor_scale = 0.5;
            let cursor_x = x - 30.;
            let cursor_y = y + text.height(ctx) / 2.
                - f32::from(asset.image.cursor.height()) * cursor_scale / 2.;

            graphics::draw(
                ctx,
                &asset.image.cursor,
                DrawParam::default()
                    .dest([cursor_x, cursor_y])
                    .scale([cursor_scale, cursor_scale]),
            )?;
        }
    }

    graphics::present(ctx)?;

    Ok(())
}
