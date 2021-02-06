use std::collections::HashMap;

use ggez::{Context, ContextBuilder, event, GameResult};
use ggez::conf::{FullscreenType, WindowMode, WindowSetup, NumSamples};
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

    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, state);
}

struct MainState {
    cursor: TitleItem,
    pressed_w_before: bool,
    pressed_s_before: bool,
    texts_ascii: Vec<graphics::Text>,
    img_cursor: graphics::Image,
    items_text_hash: HashMap<TitleItem, graphics::Text>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/Play-Regular.ttf")?;

        let ascii: Vec<&str> = r" __           __
/\ \__       /\ \__         __
\ \ ,_\    __\ \ ,_\  _ __ /\_\    ____           _ __   ____
 \ \ \/  /'__`\ \ \/ /\`'__\/\ \  /',__\  _______/\`'__\/',__\
  \ \ \_/\  __/\ \ \_\ \ \/ \ \ \/\__, `\/\______\ \ \//\__, `\
   \ \__\ \____\\ \__\\ \_\  \ \_\/\____/\/______/\ \_\\/\____/
    \/__/\/____/ \/__/ \/_/   \/_/\/___/           \/_/ \/___/"
            .split("\n")
            .collect();
        let texts_ascii: Vec<graphics::Text> = ascii.into_iter()
            .map(|line| graphics::Text::new(graphics::TextFragment::from(line)))
            .collect();

        let img_cursor = graphics::Image::new(ctx, "/cursor.png")?;

        let items_text_hash: HashMap<TitleItem, graphics::Text> = TitleItem::all()
            .into_iter()
            .map(|item| {
                let str = item.text().to_owned();

                (
                    item,
                    graphics::Text::new(
                        graphics::TextFragment::new(str)
                            .font(font)
                            .scale(PxScale::from(32.))
                    )
                )
            })
            .collect();

        Ok(
            MainState {
                cursor: TitleItem::Play40Line,
                pressed_w_before: false,
                pressed_s_before: false,
                texts_ascii,
                img_cursor,
                items_text_hash,
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

        let ascii_width = self.texts_ascii.get(4).unwrap().width(ctx);
        for (idx, text) in self.texts_ascii.iter().enumerate() {
            let x = WIDTH / 2. - ascii_width / 2.;
            let y = 50. + (15 * idx) as f32;

            graphics::draw(ctx, text, graphics::DrawParam::default().dest([x, y]))?;
        }

        for (idx, item) in TitleItem::all().iter().enumerate() {
            if let Some(text) = self.items_text_hash.get(item) {
                let x = WIDTH / 2. - text.width(ctx) / 2.;
                let y = HEIGHT / 3. + (50 * idx) as f32;

                graphics::draw(ctx, text, graphics::DrawParam::default().dest([x, y]))?;

                if item == &self.cursor {
                    let cursor_scale = 0.5f32;
                    let cursor_x = x - 30.;
                    let cursor_y = y + text.height(ctx) / 2. - f32::from(self.img_cursor.height()) * cursor_scale / 2.;

                    graphics::draw(
                        ctx,
                        &self.img_cursor,
                        graphics::DrawParam::default()
                            .dest([cursor_x, cursor_y])
                            .scale([cursor_scale, cursor_scale]),
                    )?;
                }
            }
        }

        let dbg_text = graphics::Text::new(graphics::TextFragment::new(
            format!("the cursor is at {:?}", self.cursor)
        ));
        graphics::draw(ctx, &dbg_text, graphics::DrawParam::default())?;

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