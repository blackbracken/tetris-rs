use std::convert::TryFrom;

use ggez::{Context, ContextBuilder, event, GameResult};
use ggez::conf::{FullscreenType, WindowMode};
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::timer;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 800.0;

fn main() -> GameResult {
    let cb = ContextBuilder::new("tetris-rs", "blackbracken")
        .window_mode(
            WindowMode {
                width: WIDTH,
                height: HEIGHT,
                maximized: false,
                fullscreen_type: FullscreenType::Windowed,
                borderless: false,
                min_width: 0.0,
                min_height: 0.0,
                max_width: 0.0,
                max_height: 0.0,
                resizable: false,
                visible: true,
            }
        );
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, state);
}

struct MainState {
    text: graphics::Text,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/Play-Regular.ttf")?;
        let text = graphics::Text::new(graphics::TextFragment::new("Hello, World!").font(font));

        Ok(MainState { text })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const FPS: u32 = 30;

        while timer::check_update_time(ctx, FPS) {
            // some
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(46, 46, 64));

        let ascii = r" __           __
/\ \__       /\ \__         __
\ \ ,_\    __\ \ ,_\  _ __ /\_\    ____           _ __   ____
 \ \ \/  /'__`\ \ \/ /\`'__\/\ \  /',__\  _______/\`'__\/',__\
  \ \ \_/\  __/\ \ \_\ \ \/ \ \ \/\__, `\/\______\ \ \//\__, `\
   \ \__\ \____\\ \__\\ \_\  \ \_\/\____/\/______/\ \_\\/\____/
    \/__/\/____/ \/__/ \/_/   \/_/\/___/           \/_/ \/___/";
        let split: Vec<&str> = ascii.split("\n").collect();

        let width = graphics::Text::new(
            graphics::TextFragment::new(
                split[4]
            )
        ).width(ctx);

        split
            .iter()
            .enumerate()
            .map(|(idx, line)| (idx * 15, line))
            .for_each(|(y_addition, line)| {
                let text = graphics::Text::new(graphics::TextFragment::new(line.to_owned()));
                let y = 50.0 + (y_addition as f32);

                graphics::draw(ctx, &text, graphics::DrawParam::default().dest([WIDTH / 2.0 - width / 2.0, y]));
            });

        graphics::present(ctx)?;

        Ok(())
    }
}