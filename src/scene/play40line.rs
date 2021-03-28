use std::cmp::max;
use std::time::Duration;

use ggez::{Context, GameResult, graphics};
use ggez::graphics::{DrawMode, PxScale, Rect};
use ggez::timer;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::asset::{Asset, Bgm, Se};
use crate::router::Next;
use crate::router::SceneState::ForPlay40Line;
use crate::tetris::game::{FIELD_UNIT_HEIGHT, FIELD_UNIT_WIDTH, FIELD_VISIBLE_UNIT_HEIGHT, Game, MinoBlock};

const BLOCK_LENGTH: f32 = 32.;
const HALF_BLOCK_LENGTH: f32 = BLOCK_LENGTH / 2.;

const FIELD_ORIGIN_X: f32 = WINDOW_WIDTH / 8.;
const FIELD_ORIGIN_Y: f32 = WINDOW_HEIGHT / 2. - BLOCK_LENGTH * (FIELD_VISIBLE_UNIT_HEIGHT as f32 / 2.);

pub struct Play40LineState {
    game: Game,

    countdown: Option<u64>,
    start_countdown_at: Duration,
}

impl Play40LineState {
    pub fn new(ctx: &mut Context) -> GameResult<Play40LineState> {
        Ok(
            Play40LineState {
                game: Game::new(),
                countdown: Some(3),
                start_countdown_at: timer::time_since_start(ctx),
            }
        )
    }
}

pub fn init(_ctx: &mut Context, asset: &mut Asset) {
    asset.audio.stop_bgm();
}

pub fn update(ctx: &mut Context, mut state: Play40LineState, asset: &mut Asset) -> GameResult<Next> {
    const COUNTDOWN_SEC: u64 = 3;

    let countdown = match state.countdown {
        None | Some(0) => None,
        Some(_) => {
            let diff = timer::time_since_start(ctx)
                .saturating_sub(state.start_countdown_at)
                .as_secs();
            Some(max(0, COUNTDOWN_SEC - diff))
        }
    };

    if state.countdown != countdown {
        match countdown {
            Some(0) => {
                asset.audio.play_bgm(ctx, Bgm::InGame)?;
                asset.audio.play_se(ctx, Se::GameStart)?;
            }
            Some(_) => {
                asset.audio.play_se(ctx, Se::CountdownTick)?;
            }
            None => (),
        }

        state.countdown = countdown;
    }

    Ok(Next::do_continue(ForPlay40Line { state }))
}

pub fn draw(ctx: &mut Context, state: &Play40LineState, asset: &mut Asset) -> GameResult {
    graphics::clear(ctx, asset.color.background);

    draw_field_grid(ctx, asset)?;

    match state.countdown {
        Some(0) | None => {
            draw_dropping_mino(ctx, asset, state)?;
        }
        Some(c) => {
            draw_count_down(ctx, asset, c)?;
        }
    }

    graphics::present(ctx)?;

    Ok(())
}

fn draw_field_grid(ctx: &mut Context, asset: &Asset) -> GameResult {
    const FIELD_WIDTH: f32 = BLOCK_LENGTH * (FIELD_UNIT_WIDTH as f32);
    const FIELD_HEIGHT: f32 = BLOCK_LENGTH * (FIELD_VISIBLE_UNIT_HEIGHT as f32);

    let rect = graphics::Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        Rect::new(
            FIELD_ORIGIN_X,
            FIELD_ORIGIN_Y,
            FIELD_WIDTH,
            FIELD_HEIGHT,
        ),
        asset.color.panel,
    )?;
    graphics::draw(ctx, &rect, graphics::DrawParam::default())?;

    for x in (0..=FIELD_UNIT_WIDTH).map(|x| FIELD_ORIGIN_X + (x as f32) * BLOCK_LENGTH) {
        let line = graphics::Mesh::new_line(
            ctx,
            &[
                [x, FIELD_ORIGIN_Y],
                [x, FIELD_ORIGIN_Y + FIELD_HEIGHT]
            ],
            1.,
            graphics::Color::from_rgba(24, 24, 24, 128),
        )?;
        graphics::draw(ctx, &line, graphics::DrawParam::default())?;
    }

    for y in (0..=FIELD_VISIBLE_UNIT_HEIGHT).map(|y| FIELD_ORIGIN_Y + (y as f32) * BLOCK_LENGTH) {
        let line = graphics::Mesh::new_line(
            ctx,
            &[
                [FIELD_ORIGIN_X, y],
                [FIELD_ORIGIN_X + FIELD_WIDTH, y]
            ],
            1.,
            graphics::Color::from_rgba(24, 24, 24, 128),
        )?;
        graphics::draw(ctx, &line, graphics::DrawParam::default())?;
    }

    Ok(())
}

fn draw_count_down(ctx: &mut Context, asset: &Asset, sec: u64) -> GameResult {
    let rect = graphics::Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        Rect::new(0., 0., WINDOW_WIDTH, WINDOW_HEIGHT),
        graphics::Color::new(0., 0., 0., 0.9),
    )?;

    graphics::draw(ctx, &rect, graphics::DrawParam::default())?;

    let text = graphics::Text::new(
        graphics::TextFragment::new(sec.to_string())
            .font(asset.font.vt323)
            .scale(PxScale::from(200.))
    );

    graphics::draw(
        ctx,
        &text,
        graphics::DrawParam::default()
            .dest([
                WINDOW_WIDTH / 2. - text.width(ctx) / 2.,
                WINDOW_HEIGHT / 2. - text.height(ctx) / 2.,
            ]),
    )?;

    Ok(())
}

fn draw_dropping_mino(ctx: &mut Context, asset: &mut Asset, state: &Play40LineState) -> GameResult {
    let field = state.game.board.field();
    for y in 0..FIELD_UNIT_HEIGHT {
        for x in 0..FIELD_UNIT_WIDTH {
            let block = field
                .get(y)
                .and_then(|array| array.get(x))
                .unwrap();

            if block != &MinoBlock::AIR {
                let img = asset.image.mino_block(ctx, block)?.unwrap();
                let x = x as f32;
                let y = y as f32;

                graphics::draw(
                    ctx,
                    img,
                    graphics::DrawParam::default()
                        .dest([
                            (FIELD_ORIGIN_X as f32) + (x * BLOCK_LENGTH as f32),
                            (FIELD_ORIGIN_Y as f32) + (y * BLOCK_LENGTH as f32)
                        ]),
                )?;
            }
        }
    }

    Ok(())
}