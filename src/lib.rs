#![feature(duration_zero)]
#![feature(duration_saturating_ops)]
#![feature(duration_consts_2)]
#![feature(pub_macro_rules)]
#![feature(map_into_keys_values)]

use std::mem;
use std::time::Duration;

use ggez::{Context, ContextBuilder, event, GameResult};
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::timer;

use crate::asset::Asset;
use crate::router::{Next, SceneState, Ticket};

mod macros;
mod scene;
mod router;
mod asset;
mod tetris;
mod input;

pub const FPS: u32 = 60;
pub const WINDOW_WIDTH: f32 = 960.;
pub const WINDOW_HEIGHT: f32 = 800.;

pub fn start(cb: ContextBuilder) -> GameResult {
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state);
}

struct MainState {
    scene_state: Option<SceneState>,
    asset: Box<Asset>,
    last_measured: Duration,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut asset = Asset::load(ctx)?;

        Ok(
            MainState {
                scene_state: Some(Ticket::ShowTitle.go(ctx, &mut asset)?),
                asset,
                last_measured: timer::time_since_start(ctx),
            }
        )
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, FPS) {
            let state = mem::replace(&mut self.scene_state, None)
                .expect("scene_state has not been updated");

            let now = timer::time_since_start(ctx);
            let diff = now - self.last_measured;
            self.last_measured = now;

            let next: Next = match state {
                SceneState::ForTitle { state } => {
                    scene::title::update(ctx, state, &self.asset)?
                }
                SceneState::ForPlay40Line { state } => {
                    scene::play40line::update(ctx, state, &mut self.asset, diff)?
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

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        // disable a function to quit on pushing the escape key.
    }
}