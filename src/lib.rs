#![feature(duration_consts_float)]
#![feature(never_type)]

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate num_derive;

use std::mem;
use std::time::Duration;

use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::timer;
use ggez::{event, Context, ContextBuilder, GameResult};

use scene::ticket;

use crate::asset::Asset;
use crate::scene::scene_state::SceneState;
use crate::ticket::{Next, Ticket};

mod domain;
mod infra;
mod input;
mod macros;

pub mod scene;

pub(crate) mod scenes {
    pub mod router;

    pub mod play40line;
    pub mod title;
}

pub(crate) mod tetris {
    pub mod board;
    pub mod game;
    pub mod mino_bag;

    pub mod model {
        pub mod mino_entity;
        pub mod score;
        pub mod spin;
        pub mod tetrimino;
    }
}

pub(crate) mod asset;

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

        Ok(MainState {
            scene_state: Some(Ticket::ShowTitle.go(ctx, &mut asset)?),
            asset,
            last_measured: timer::time_since_start(ctx),
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, FPS) {
            let state = mem::take(&mut self.scene_state).unwrap();

            let now = timer::time_since_start(ctx);
            let diff = now - mem::replace(&mut self.last_measured, now);

            let next: Next = match state {
                SceneState::ForTitle { state } => scene::title::title_scene::update(ctx, state)?,
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
                SceneState::ForTitle { state } => {
                    scene::title::title_scene::draw(ctx, state, &mut self.asset)?;
                }
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
        // disable a function to quit on pushing the escape key
    }
}
