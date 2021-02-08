use std::collections::HashMap;

use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Color, PxScale};
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;

use crate::{HEIGHT, WIDTH};
use crate::router::Next;
use crate::router::ViewState::ForTitle;

#[derive(Clone)]
pub struct TitleState {
    cursor: TitleItem,
    pressed_up_before: bool,
    pressed_down_before: bool,
    texts_ascii: Vec<graphics::Text>,
    img_cursor: graphics::Image,
    items_text_hash: HashMap<TitleItem, graphics::Text>,
}

impl TitleState {
    pub fn new(ctx: &mut Context) -> GameResult<TitleState> {
        let font = graphics::Font::new(ctx, "/Play-Regular.ttf")?;

        let ascii: Vec<&str> = r" __           __
/\ \__       /\ \__         __
\ \ ,_\    __\ \ ,_\  _ __ /\_\    ____           _ __   ____
 \ \ \/  /'__`\ \ \/ /\`'__\/\ \  /',__\  _______/\`'__\/',__\
  \ \ \_/\  __/\ \ \_\ \ \/ \ \ \/\__, `\/\______\ \ \//\__, `\
   \ \__\ \____\\ \__\\ \_\  \ \_\/\____/\/______/\ \_\\/\____/
    \/__/\/____/ \/__/ \/_/   \/_/\/___/           \/_/ \/___/"
            .split("\n")
            .collect();
        let texts_ascii: Vec<graphics::Text> = ascii.into_iter()
            .map(|line| graphics::Text::new(graphics::TextFragment::from(line)))
            .collect();

        let img_cursor = graphics::Image::new(ctx, "/cursor.png")?;

        let items_text_hash: HashMap<TitleItem, graphics::Text> = TitleItem::all()
            .into_iter()
            .map(|item| {
                let str = item.text().to_owned();

                (
                    item,
                    graphics::Text::new(
                        graphics::TextFragment::new(str)
                            .font(font)
                            .scale(PxScale::from(32.))
                    )
                )
            })
            .collect();

        Ok(
            TitleState {
                cursor: TitleItem::Play40Line,
                pressed_up_before: false,
                pressed_down_before: false,
                texts_ascii,
                img_cursor,
                items_text_hash,
            }
        )
    }
}

pub fn update(ctx: &Context, state: &TitleState) -> Next {
    // TODO: refactor to remove mutable actions
    let mut new_state = state.clone();

    let pressed_up = keyboard::is_key_pressed(ctx, KeyCode::W);
    if pressed_up && !state.pressed_up_before {
        if let Some(prev) = state.cursor.prev() {
            new_state.cursor = prev;
        }
    }
    new_state.pressed_up_before = pressed_up;

    let pressed_down = keyboard::is_key_pressed(ctx, KeyCode::S);
    if pressed_down && !new_state.pressed_down_before {
        if let Some(next) = new_state.cursor.next() {
            new_state.cursor = next;
        }
    }
    new_state.pressed_down_before = pressed_down;

    Next::do_continue(ForTitle { state: new_state })
}

pub fn draw(ctx: &mut Context, state: &TitleState) -> GameResult {
    graphics::clear(ctx, Color::from_rgb(46, 46, 46));

    let ascii_width = state.texts_ascii.get(4).unwrap().width(ctx);
    for (idx, text) in state.texts_ascii.iter().enumerate() {
        let x = WIDTH / 2. - ascii_width / 2.;
        let y = 50. + (15 * idx) as f32;

        graphics::draw(ctx, text, graphics::DrawParam::default().dest([x, y]))?;
    }

    for (idx, item) in TitleItem::all().iter().enumerate() {
        if let Some(text) = state.items_text_hash.get(item) {
            let x = WIDTH / 2. - text.width(ctx) / 2.;
            let y = HEIGHT / 3. + (50 * idx) as f32;

            graphics::draw(ctx, text, graphics::DrawParam::default().dest([x, y]))?;

            if item == &state.cursor {
                let cursor_scale = 0.5f32;
                let cursor_x = x - 30.;
                let cursor_y = y + text.height(ctx) / 2. - f32::from(state.img_cursor.height()) * cursor_scale / 2.;

                graphics::draw(
                    ctx,
                    &state.img_cursor,
                    graphics::DrawParam::default()
                        .dest([cursor_x, cursor_y])
                        .scale([cursor_scale, cursor_scale]),
                )?;
            }
        }
    }

    let dbg_text = graphics::Text::new(graphics::TextFragment::new(
        format!("the cursor is at {:?}", state.cursor)
    ));
    graphics::draw(ctx, &dbg_text, graphics::DrawParam::default())?;

    graphics::present(ctx)?;

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum TitleItem {
    Play40Line,
    Exit,
}

impl TitleItem {
    fn next(&self) -> Option<TitleItem> {
        match &self {
            TitleItem::Play40Line => Some(TitleItem::Exit),
            TitleItem::Exit => None,
        }
    }

    fn prev(&self) -> Option<TitleItem> {
        match &self {
            TitleItem::Play40Line => None,
            TitleItem::Exit => Some(TitleItem::Play40Line),
        }
    }

    fn text(&self) -> &str {
        match &self {
            TitleItem::Play40Line => "Play 40LINE",
            TitleItem::Exit => "Exit",
        }
    }

    fn all() -> Vec<TitleItem> {
        vec![TitleItem::Play40Line, TitleItem::Exit]
    }
}