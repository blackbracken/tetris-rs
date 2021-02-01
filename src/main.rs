use ggez::{Context, ContextBuilder, event, GameResult};
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::timer;
use std::convert::TryFrom;

fn main() -> GameResult {
    let cb = ContextBuilder::new("tetris-rs", "blackbracken");
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

        ascii.split("\n").enumerate()
            .map(|(idx, line)| (idx * 15, line))
            .for_each(|(y_addition, line)| {
                let tex = graphics::Text::new(graphics::TextFragment::new(line));
                let y = 60f32 + (y_addition as f32);
                graphics::draw(ctx, &tex, graphics::DrawParam::default().dest([0f32, y]));
            });
        graphics::present(ctx)?;

        Ok(())
    }
}