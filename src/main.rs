use ggez::{Context, ContextBuilder, event, GameResult};
use ggez::conf::{FullscreenType, NumSamples, WindowMode, WindowSetup};
use ggez::event::EventHandler;
use ggez::timer;

use crate::resource::SharedResource;
use crate::router::{Next, SceneState, Ticket};

mod scene;
mod router;
mod resource;
mod tetris;

pub const FPS: u32 = 60;
pub const WIDTH: f32 = 640.;
pub const HEIGHT: f32 = 800.;

fn main() -> GameResult {
    let cb = ContextBuilder::new("tetris-rs", "blackbracken")
        .window_mode(
            WindowMode {
                width: WIDTH,
                height: HEIGHT,
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
    scene_state: SceneState,
    resource: Box<SharedResource>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut resource = SharedResource::load(ctx)?;

        Ok(
            MainState {
                scene_state: Ticket::ShowTitle.go(ctx, &mut resource)?,
                resource,
            }
        )
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, FPS) {
            let next: Next = match &self.scene_state {
                SceneState::ForTitle { state } => scene::title::update(ctx, state, &self.resource),
                SceneState::ForPlay40Line { state } => scene::play40line::update(ctx, state),
            };

            match next {
                Next::Continue { state } => {
                    self.scene_state = state;
                }
                Next::Transit { ticket } => {
                    self.scene_state = ticket.go(ctx, &mut self.resource)?;
                }
                Next::Exit => {
                    event::quit(ctx);
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &self.scene_state {
            SceneState::ForTitle { state } => { scene::title::draw(ctx, state, &self.resource)?; }
            SceneState::ForPlay40Line { state } => { scene::play40line::draw(ctx, state, &self.resource)?; }
        }

        Ok(())
    }
}