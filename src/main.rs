use ggez::{Context, ContextBuilder, event, GameResult};
use ggez::conf::{FullscreenType, NumSamples, WindowMode, WindowSetup};
use ggez::event::EventHandler;
use ggez::timer;

use crate::resource::SharedResource;
use crate::router::{Next, Ticket, ViewState};

mod view;
mod router;
mod resource;

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
    view_state: ViewState,
    resource: Box<SharedResource>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let resource = SharedResource::load(ctx)?;

        Ok(
            MainState {
                view_state: Ticket::ShowTitle.go(ctx, &resource)?,
                resource,
            }
        )
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const FPS: u32 = 30;

        while timer::check_update_time(ctx, FPS) {
            let next: Next = match &self.view_state {
                ViewState::ForTitle { state } => view::title::update(ctx, state),
                ViewState::ForPlay40Line { state } => view::play40line::update(ctx, state),
            };

            match next {
                Next::Continue { state } => {
                    self.view_state = state;
                }
                Next::Transit { ticket } => {
                    self.view_state = ticket.go(ctx, &self.resource)?;
                }
                Next::Exit => {
                    event::quit(ctx);
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &self.view_state {
            ViewState::ForTitle { state } => { view::title::draw(ctx, state, &self.resource)?; }
            ViewState::ForPlay40Line { state } => { view::play40line::draw(ctx, state, &self.resource)?; }
        }

        Ok(())
    }
}