#![feature(duration_zero)]
#![feature(duration_saturating_ops)]

use std::mem;

use ggez::{Context, ContextBuilder, event, GameResult};
use ggez::conf::{FullscreenType, NumSamples, WindowMode, WindowSetup};
use ggez::event::EventHandler;
use ggez::timer;

use crate::asset::Asset;
use crate::router::{Next, SceneState, Ticket};

mod scene;
mod router;
mod asset;
mod tetris;

pub const FPS: u32 = 60;
pub const WINDOW_WIDTH: f32 = 960.;
pub const WINDOW_HEIGHT: f32 = 800.;

fn main() -> GameResult {
    let cb = ContextBuilder::new("tetris-rs", "blackbracken")
        .window_mode(
            WindowMode {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                maximized: false,
                fullscreen_type: FullscreenType::Windowed,
                borderless: false,
                min_width: 0.,
                min_height: 0.,
                max_width: 0.,
                max_height: 0.,
                resizable: false,
                visible: true,
            }
        )
        .window_setup(
            WindowSetup {
                title: "tetris-rs".to_owned(),
                samples: NumSamples::Zero,
                vsync: true,
                icon: "".to_owned(),
                srgb: false,
            }
        );

    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state);
}

struct MainState {
    scene_state: Option<SceneState>,
    asset: Box<Asset>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut asset = Asset::new(ctx)?;

        Ok(
            MainState {
                scene_state: Some(Ticket::ShowTitle.go(ctx, &mut asset)?),
                asset,
            }
        )
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, FPS) {
            let state = mem::replace(&mut self.scene_state, None)
                .expect("scene_state has not been updated");

            let next: Next = match state {
                SceneState::ForTitle { state } => {
                    scene::title::update(ctx, state, &self.asset)?
                }
                SceneState::ForPlay40Line { state } => {
                    scene::play40line::update(ctx, state, &mut self.asset)?
                }
            };

            match next {
                Next::Continue { state } => {
                    self.scene_state = Some(state);
                }
                Next::Transit { ticket } => {
                    self.scene_state = Some(ticket.go(ctx, &mut self.asset)?);
                }
                Next::Exit => {
                    event::quit(ctx);
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(state) = &self.scene_state {
            match state {
                SceneState::ForTitle { state } => { scene::title::draw(ctx, state, &self.asset)?; }
                SceneState::ForPlay40Line { state } => { scene::play40line::draw(ctx, state, &mut self.asset)?; }
            }
        }

        Ok(())
    }
}