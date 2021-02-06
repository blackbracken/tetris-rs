use std::collections::{HashMap};
use ggez::{Context, ContextBuilder, event, GameResult};
use ggez::conf::{FullscreenType, WindowMode};
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics;
use ggez::graphics::{Color, PxScale};
use ggez::input::keyboard;
use ggez::timer;

const WIDTH: f32 = 640.;
const HEIGHT: f32 = 800.;

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
        );
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, state);
}

struct MainState {
    cursor: TitleItem,
    pressed_w_before: bool,
    pressed_s_before: bool,
    img_cursor: graphics::Image,
    text_hash: HashMap<TitleItem, graphics::Text>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/Play-Regular.ttf")?;
        let img_cursor = graphics::Image::new(ctx, "/cursor.png")?;

        let mut text_hash: HashMap<TitleItem, graphics::Text> = HashMap::new();
        TitleItem::all().into_iter().for_each(|item: TitleItem| {
            let text = item.text().to_owned();

            text_hash.insert(item, graphics::Text::new(graphics::TextFragment::new(text).font(font).scale(PxScale::from(32.))));
        });

        Ok(
            MainState {
                cursor: TitleItem::Play40Line,
                pressed_w_before: false,
                pressed_s_before: false,
                img_cursor,
                text_hash,
            }
        )
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const FPS: u32 = 30;

        while timer::check_update_time(ctx, FPS) {
            let pressed_w = keyboard::is_key_pressed(ctx, KeyCode::W);
            if pressed_w && !self.pressed_w_before {
                if let Some(prev) = self.cursor.prev() {
                    self.cursor = prev;
                }
            }
            self.pressed_w_before = pressed_w;

            let pressed_s = keyboard::is_key_pressed(ctx, KeyCode::S);
            if pressed_s && !self.pressed_s_before {
                if let Some(next) = self.cursor.next() {
                    self.cursor = next;
                }
            }
            self.pressed_s_before = pressed_s;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(46, 46, 46));

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
            .for_each(|(idx, line)| {
                let text = graphics::Text::new(graphics::TextFragment::new(line.to_owned()));
                let x = WIDTH / 2. - width / 2.;
                let y = 50. + (15 * idx) as f32;

                graphics::draw(ctx, &text, graphics::DrawParam::default().dest([x, y]));
            });

        TitleItem::all()
            .iter()
            .enumerate()
            .for_each(|(idx, item)| {
                if let Some(text) = self.text_hash.get(item) {
                    let x = WIDTH / 2. - text.width(ctx) / 2.;
                    let y = HEIGHT / 3. + (50 * idx) as f32;

                    graphics::draw(ctx, text, graphics::DrawParam::default().dest([x, y]));

                    if item == &self.cursor {
                        graphics::draw(
                            ctx,
                            &self.img_cursor,
                            graphics::DrawParam::default().dest([x - 30., y + text.height(ctx) / 2. - f32::from(self.img_cursor.height()) * 0.5 / 2.]).scale([0.5, 0.5]),
                        );
                    }
                }
            });

        let raw_text = format!("the cursor is at {:?}", self.cursor);
        let text = graphics::Text::new(graphics::TextFragment::new(raw_text));
        graphics::draw(ctx, &text, graphics::DrawParam::default());

        graphics::present(ctx)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum TitleItem {
    Play40Line,
    Exit,
}

impl TitleItem {
    fn next(&self) -> Option<TitleItem> {
        match &self {
            TitleItem::Play40Line => Some(TitleItem::Exit),
            TitleItem::Exit => None,
        }
    }

    fn prev(&self) -> Option<TitleItem> {
        match &self {
            TitleItem::Play40Line => None,
            TitleItem::Exit => Some(TitleItem::Play40Line),
        }
    }

    fn text(&self) -> &str {
        match &self {
            TitleItem::Play40Line => "Play 40LINE",
            TitleItem::Exit => "Exit",
        }
    }

    fn all() -> Vec<TitleItem> {
        vec![TitleItem::Play40Line, TitleItem::Exit]
    }
}

