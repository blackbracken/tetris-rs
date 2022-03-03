#![feature(duration_consts_float)]
#![feature(never_type)]
#![feature(variant_count)]
#![feature(map_try_insert)]

use std::{mem, time::Duration};

use ggez::{
    event,
    event::{EventHandler, KeyCode, KeyMods},
    input::{gamepad::gamepads, keyboard},
    timer,
    Context,
    ContextBuilder,
    GameResult,
};

use crate::{
    asset::Asset,
    infra::repo::{
        asset_provider_impl::AssetProviderImpl,
        control_code_repository_impl::ControlCodeRepositoryImpl,
    },
    kernel::{
        input::{control_code::ControlCode, input_cache::InputCache},
        repo::{asset_provider::AssetProvider, control_code_repository::ControlCodeRepository},
    },
    scene::{scene_state::SceneState, ticket},
    ticket::{Next, Ticket},
};

mod asset;
mod infra;
mod kernel;
mod scene;

pub const FPS: u32 = 60;
pub const WINDOW_WIDTH: f32 = 960.;
pub const WINDOW_HEIGHT: f32 = 800.;

pub fn start(cb: ContextBuilder) -> GameResult {
    let (mut ctx, event_loop) = cb.build()?;

    let control_code_repo = ControlCodeRepositoryImpl::new();
    let asset_provider = AssetProviderImpl::new();

    let state = MainStateHolder::new(&mut ctx, control_code_repo, asset_provider)?;

    event::run(ctx, event_loop, state);
}

trait MainState
where
    Self: Sized,
{
    type CCR: ControlCodeRepository;
    type AP: AssetProvider;

    fn new(
        ctx: &mut Context,
        control_code_repo: Self::CCR,
        asset_provider: Self::AP,
    ) -> GameResult<Self>;

    fn find_input(&self, ctx: &Context) -> Vec<ControlCode>;
}

struct MainStateHolder {
    scene_state: Option<SceneState>,
    asset: Box<Asset>,
    last_measured: Duration,
    input_cache: InputCache,
    control_code_repo: ControlCodeRepositoryImpl,
    asset_provider: AssetProviderImpl,
}

impl MainState for MainStateHolder {
    type CCR = ControlCodeRepositoryImpl;
    type AP = AssetProviderImpl;

    fn new(
        ctx: &mut Context,
        control_code_repo: Self::CCR,
        asset_provider: Self::AP,
    ) -> GameResult<Self> {
        let mut asset = Asset::load(ctx)?;

        Ok(MainStateHolder {
            scene_state: Some(Ticket::ShowTitle.go(ctx, &mut asset)?),
            asset,
            last_measured: timer::time_since_start(ctx),
            input_cache: InputCache::new(),
            control_code_repo,
            asset_provider,
        })
    }

    fn find_input(&self, ctx: &Context) -> Vec<ControlCode> {
        let control_code_repo = &self.control_code_repo;

        ControlCode::all()
            .into_iter()
            .filter(|code| {
                let keys = control_code_repo.key_codes(code);
                let buttons = control_code_repo.buttons(code);

                let pressed_key = keys.iter().any(|&key| keyboard::is_key_pressed(ctx, key));
                let pressed_button = buttons
                    .iter()
                    .any(|&btn| gamepads(ctx).any(|(_, pad)| pad.is_pressed(btn)));

                pressed_key || pressed_button
            })
            .collect()
    }
}

impl EventHandler for MainStateHolder {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, FPS) {
            let scene_state = mem::take(&mut self.scene_state).unwrap();

            let now = timer::time_since_start(ctx);
            let delta = now - mem::replace(&mut self.last_measured, now);

            let inputs = self.find_input(ctx);
            self.input_cache.receive_inputs(&inputs, &delta);

            let next: Next = match scene_state {
                SceneState::ForTitle { state } => {
                    scene::title::title_scene::update(ctx, &mut self.input_cache, state, &delta)?
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
                SceneState::ForTitle { state } => {
                    scene::title::title_scene::draw(
                        ctx,
                        state,
                        &mut self.asset,
                        &mut self.asset_provider,
                    )?;
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
