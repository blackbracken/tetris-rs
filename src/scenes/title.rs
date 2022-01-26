use std::collections::HashMap;

use ggez::{
    graphics,
    graphics::{Color, PxScale},
    Context,
    GameResult,
};
use rand::random;

use crate::{
    asset::{
        audio::{Bgm, Se},
        Asset,
    },
    input::{pressed_down, pressed_enter, pressed_up},
    scenes::router::{Next, Ticket},
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

#[derive(Clone)]
pub struct TitleState {
    cursor: TitleItem,
    pressed_up_before: bool,
    pressed_down_before: bool,
    texts_ascii: Vec<graphics::Text>,
    items_text_hash: HashMap<TitleItem, graphics::Text>,
    particles: Vec<TitleParticle>,
}

impl TitleState {
    pub fn new(_ctx: &mut Context, asset: &mut Asset) -> GameResult<TitleState> {
        let ascii: Vec<&str> = r" __           __
/\ \__       /\ \__         __
\ \ ,_\    __\ \ ,_\  _ __ /\_\    ____           _ __   ____
 \ \ \/  /'__`\ \ \/ /\`'__\/\ \  /',__\  _______/\`'__\/',__\
  \ \ \_/\  __/\ \ \_\ \ \/ \ \ \/\__, `\/\______\ \ \//\__, `\
   \ \__\ \____\\ \__\\ \_\  \ \_\/\____/\/______/\ \_\\/\____/
    \/__/\/____/ \/__/ \/_/   \/_/\/___/           \/_/ \/___/"
            .split("\n")
            .collect();
        let texts_ascii: Vec<graphics::Text> = ascii
            .into_iter()
            .map(|line| graphics::Text::new(graphics::TextFragment::from(line)))
            .collect();

        let items_text_hash: HashMap<TitleItem, graphics::Text> = TitleItem::all()
            .into_iter()
            .map(|item| {
                let str = item.text().to_owned();

                (
                    item,
                    graphics::Text::new(
                        graphics::TextFragment::new(str)
                            .font(asset.font.play)
                            .scale(PxScale::from(32.)),
                    ),
                )
            })
            .collect();

        let particles = (0..32)
            .map(|_| {
                TitleParticle::new(
                    (WINDOW_WIDTH + 20.) * (random::<f32>() % 1.) - 10.,
                    WINDOW_HEIGHT * (random::<f32>() % 1.),
                )
            })
            .collect::<Vec<_>>();

        Ok(TitleState {
            cursor: TitleItem::Play40Line,
            pressed_up_before: false,
            pressed_down_before: false,
            texts_ascii,
            items_text_hash,
            particles,
        })
    }
}

pub fn init(ctx: &mut Context, asset: &mut Asset) -> GameResult {
    asset.audio.play_bgm(ctx, Bgm::Title)?;

    Ok(())
}

pub fn update(ctx: &mut Context, mut state: TitleState, asset: &Asset) -> GameResult<Next> {
    for particle in &mut state.particles {
        particle.y -= particle.up_speed;
        particle.rotation += particle.rotate_speed;
    }
    if random::<u32>() % (crate::FPS / 4) == 0 {
        state.particles.push(TitleParticle::new(
            (WINDOW_WIDTH + 20.) * (random::<f32>() % 1.) - 10.,
            WINDOW_HEIGHT + 10.,
        ));
    }
    state.particles.retain(|&particle| particle.y > -30.);

    let pressed_up = pressed_up(ctx);
    if pressed_up && !state.pressed_up_before {
        asset.audio.play_se(ctx, Se::MenuClick)?;

        if let Some(prev) = state.cursor.prev() {
            state.cursor = prev;
        }
    }
    state.pressed_up_before = pressed_up;

    let pressed_down = pressed_down(ctx);
    if pressed_down && !state.pressed_down_before {
        asset.audio.play_se(ctx, Se::MenuClick)?;

        if let Some(next) = state.cursor.next() {
            state.cursor = next;
        }
    }
    state.pressed_down_before = pressed_down;

    if pressed_enter(ctx) {
        asset.audio.play_se(ctx, Se::MenuClick)?;

        Ok(match state.cursor {
            TitleItem::Play40Line => Next::transit(Ticket::Play40Line),
            TitleItem::Exit => Next::Exit,
        })
    } else {
        Ok(Next::do_continue(state.into()))
    }
}

pub fn draw(ctx: &mut Context, state: &TitleState, asset: &Asset) -> GameResult {
    graphics::clear(ctx, asset.color.background);

    for particle in &state.particles {
        graphics::draw(
            ctx,
            &asset.image.title_particle,
            graphics::DrawParam::default()
                .color(Color::new(1., 1., 1., 0.2))
                .dest([particle.x, particle.y])
                .scale([particle.size, particle.size])
                .rotation(particle.rotation)
                .offset([0.5, 0.5]),
        )?;
    }

    let ascii_width = state.texts_ascii.get(4).unwrap().width(ctx);
    for (idx, text) in state.texts_ascii.iter().enumerate() {
        let x = WINDOW_WIDTH / 2. - ascii_width / 2.;
        let y = 50. + (15 * idx) as f32;

        graphics::draw(ctx, text, graphics::DrawParam::default().dest([x, y]))?;
    }

    for (idx, item) in TitleItem::all().iter().enumerate() {
        if let Some(text) = state.items_text_hash.get(item) {
            let x = WINDOW_WIDTH / 2. - text.width(ctx) / 2.;
            let y = WINDOW_HEIGHT / 3. + (50 * idx) as f32;

            graphics::draw(ctx, text, graphics::DrawParam::default().dest([x, y]))?;

            if item == &state.cursor {
                let cursor_scale = 0.5f32;
                let cursor_x = x - 30.;
                let cursor_y = y + text.height(ctx) / 2.
                    - f32::from(asset.image.cursor.height()) * cursor_scale / 2.;

                graphics::draw(
                    ctx,
                    &asset.image.cursor,
                    graphics::DrawParam::default()
                        .dest([cursor_x, cursor_y])
                        .scale([cursor_scale, cursor_scale]),
                )?;
            }
        }
    }

    let dbg_text = graphics::Text::new(graphics::TextFragment::new(format!(
        "the cursor is at {:?}",
        state.cursor
    )));
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

#[derive(Copy, Clone)]
struct TitleParticle {
    x: f32,
    y: f32,
    up_speed: f32,
    rotate_speed: f32,
    size: f32,
    rotation: f32,
}

impl TitleParticle {
    fn new(x: f32, y: f32) -> TitleParticle {
        TitleParticle {
            x,
            y,
            up_speed: 0.8 + (random::<f32>() % 4.),
            rotate_speed: random::<f32>() % 0.09,
            size: 0.4 + (random::<f32>() % 0.4),
            rotation: random::<f32>() % 360.,
        }
    }
}
