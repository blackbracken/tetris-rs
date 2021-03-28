use std::cmp::{max, min};
use std::ops::Sub;
use std::time::Duration;

use ggez::{Context, GameResult, graphics};
use ggez::graphics::{DrawMode, FilterMode, PxScale, Rect};
use ggez::timer;

use crate::{HEIGHT, WIDTH};
use crate::asset::{Asset, Se};
use crate::router::Next;
use crate::router::SceneState::ForPlay40Line;
use crate::tetris::game::{Game, MinoBlock};

const BLOCK_LENGTH: f32 = 32.;

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

pub fn update(ctx: &mut Context, mut state: Play40LineState, asset: &Asset) -> Next {
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
        println!("countdown is ${:?}", countdown);
        match countdown {
            Some(0) => asset.audio.play_se(ctx, Se::GameStart),
            Some(_) => asset.audio.play_se(ctx, Se::CountdownTick),
            None => (),
        }

        state.countdown = countdown;
    }

    Next::do_continue(ForPlay40Line { state })
}

pub fn draw(ctx: &mut Context, state: &Play40LineState, asset: &mut Asset) -> GameResult {
    graphics::clear(ctx, asset.color.background);

    for x in (0..=10).map(|x| (x as f32) * BLOCK_LENGTH) {
        let line = graphics::Mesh::new_line(
            ctx,
            &[[x, 0.], [x, BLOCK_LENGTH * 22.]],
            2.,
            graphics::Color::from_rgb(24, 24, 24),
        )?;
        graphics::draw(ctx, &line, graphics::DrawParam::default());
    }

    for y in (0..=22).map(|y| (y as f32) * BLOCK_LENGTH) {
        let line = graphics::Mesh::new_line(
            ctx,
            &[[0., y], [BLOCK_LENGTH * 10., y]],
            2.,
            graphics::Color::from_rgb(24, 24, 24),
        )?;
        graphics::draw(ctx, &line, graphics::DrawParam::default());
    }

    match state.countdown {
        Some(0) | None => {
            let field = state.game.board.field();
            for y in 0..20 {
                for x in 0..10 {
                    let block = field
                        .get(y)
                        .and_then(|array| array.get(x))
                        .unwrap();

                    if block != &MinoBlock::AIR {
                        let img = asset.image.mino_block(ctx, block)?;
                        graphics::draw(
                            ctx,
                            img,
                            graphics::DrawParam::default()
                                .dest([(x * 32) as f32, (y * 32) as f32]),
                        );
                    }
                }
            }
        }
        Some(c) => {
            draw_count_down(ctx, asset, c);
        }
    }

    graphics::present(ctx)?;

    Ok(())
}

fn draw_count_down(ctx: &mut Context, asset: &Asset, sec: u64) -> GameResult {
    let rect = graphics::Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        Rect::new(0., 0., WIDTH, HEIGHT),
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
                WIDTH / 2. - text.width(ctx) / 2.,
                HEIGHT / 2. - text.height(ctx) / 2.,
            ]),
    );

    Ok(())
}