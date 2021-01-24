extern crate ncurses;

use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;
use ncurses::{endwin, getch, refresh};

use crate::grap::cui::init_ncurses;
use crate::scene::{Destination, InputAction, Title, TitleItem};
use crate::scene::title::cui::CuiTitle;
use ggez::graphics::TextFragment;
use ggez::conf::{WindowSetup, NumSamples};

mod scene;
mod grap;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("hello", "blackbracken")
        .window_setup(WindowSetup {
            title: "sample".to_owned(),
            samples: NumSamples::Zero,
            vsync: true,
            icon: "".to_string(),
            srgb: false
        });
    let (mut ctx, event_loop) = cb.build()?;

    let game = Game::new(&mut ctx)?;
    ggez::event::run(ctx, event_loop, game)
}

struct Game { frames: usize, text: graphics::Text }

impl Game {
    pub fn new(_ctx: &mut Context) -> GameResult<Game> {
        Ok(Game { frames: 0, text: graphics::Text::new(TextFragment::new("hello")) })
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const FPS: u32 = 30;
        while ggez::timer::check_update_time(ctx, FPS) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.frames += 1;
        if (self.frames % 60) == 0 {
            println!("your fps: {}", ggez::timer::fps(ctx));
            self.frames = 0;
        }

        graphics::clear(ctx, graphics::WHITE);
        graphics::present(ctx);

        Ok(())
    }
}