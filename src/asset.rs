#![feature(once_cell)]

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::time::Duration;

use ggez::{audio, Context, GameError, GameResult, graphics};
use ggez::audio::SoundSource;
use ggez::graphics::{GlBackendSpec, ImageGeneric};

use crate::tetris::game::MinoBlock;

const BGM_VOLUME: f32 = 0.2;
const SE_VOLUME: f32 = 0.4;

pub struct Asset {
    pub image: Image,
    pub audio: Audio,
    pub font: Font,
    pub color: Color,
}

impl Asset {
    // TODO: support asynchronous loading
    pub fn new(ctx: &mut Context) -> GameResult<Box<Asset>> {
        Ok(
            Box::new(
                Asset {
                    image: Image::new(ctx)?,
                    audio: Audio::new(ctx)?,
                    font: Font::new(ctx)?,
                    color: Color::new(),
                }
            )
        )
    }
}

pub struct Image {
    pub cursor: graphics::Image,
    pub title_particle: graphics::Image,
    mino_block: graphics::Image,
    mino_block_images: HashMap<MinoBlock, graphics::Image>,
}

impl Image {
    fn new(ctx: &mut Context) -> GameResult<Image> {
        Ok(
            Image {
                cursor: graphics::Image::new(ctx, "/image/cursor.png")?,
                title_particle: graphics::Image::new(ctx, "/image/particles/title.png")?,
                mino_block: graphics::Image::new(ctx, "/image/mino_block.png")?,
                mino_block_images: HashMap::new(),
            }
        )
    }

    pub fn mino_block<'a>(&'a mut self, ctx: &mut Context, block: &MinoBlock) -> GameResult<&'a graphics::Image> {
        if self.mino_block_images.get(block) == None {
            let img = self.colorize(ctx, block)?;
            let _ = self.mino_block_images.insert(block.clone(), img);
        }

        Ok(self.mino_block_images.get(block).unwrap())
    }

    fn colorize(&self, ctx: &mut Context, block: &MinoBlock) -> GameResult<graphics::Image> {
        const RED: usize = 0;
        const GREEN: usize = 1;
        const BLUE: usize = 2;
        const ALPHA: usize = 3;

        let w = self.mino_block.width();
        let h = self.mino_block.height();

        let rgba = self.mino_block
            .to_rgba8(ctx)?
            .iter()
            .enumerate()
            .map(|(idx, &v)| match block {
                MinoBlock::PURPLE => match idx % 4 {
                    RED | BLUE => v.saturating_add(80),
                    GREEN => v.saturating_sub(64),
                    ALPHA => 255u8,
                    _ => v,
                }
                MinoBlock::AIR => match idx % 4 {
                    RED | GREEN | BLUE => v.saturating_add(16),
                    ALPHA => 255u8,
                    _ => v
                }
                _ => unimplemented!(),
            })
            .collect::<Vec<_>>();

        graphics::Image::from_rgba8(ctx, w, h, rgba.as_slice())
    }
}

pub struct Audio {
    bgm_data_map: HashMap<Bgm, audio::SoundData>,
    se_data_map: HashMap<Se, audio::SoundData>,

    playing_src: Option<audio::Source>,
}

impl Audio {
    fn new(ctx: &mut Context) -> GameResult<Audio> {
        // for exhaustive checking on compile
        fn bgm_path(bgm: Bgm) -> &'static str {
            match bgm {
                Bgm::Title => "/sound/bgm/bgm_maoudamashii_cyber18.mp3",
                Bgm::InGame => "/sound/bgm/game_maoudamashii_7_rock44.mp3"
            }
        }
        let bgm_data_map = maplit::hashmap! {
            Bgm::Title => audio::SoundData::new(ctx, bgm_path(Bgm::Title))?,
            Bgm::InGame => audio::SoundData::new(ctx, bgm_path(Bgm::InGame))?,
        };

        // for exhaustive checking on compile
        fn se_path(se: Se) -> &'static str {
            match se {
                Se::MenuClick => "/sound/se/se_maoudamashii_system26.mp3",
                Se::CountdownTick => "/sound/se/se_maoudamashii_instruments_drum1_hat.mp3",
                Se::GameStart => "/sound/se/se_maoudamashii_instruments_drum1_tom3.mp3",
            }
        }
        let se_data_map = maplit::hashmap! {
            Se::MenuClick => audio::SoundData::new(ctx, se_path(Se::MenuClick))?,
            Se::CountdownTick => audio::SoundData::new(ctx, se_path(Se::CountdownTick))?,
            Se::GameStart => audio::SoundData::new(ctx, se_path(Se::GameStart))?,
        };

        Ok(
            Audio {
                bgm_data_map,
                se_data_map,
                playing_src: None,
            }
        )
    }

    pub fn play_bgm(&mut self, ctx: &mut Context, bgm: Bgm) {
        self.stop_bgm();

        let src = self.bgm_data_map.get(&bgm)
            .and_then(|data| audio::Source::from_data(ctx, data.clone()).ok())
            .map(|mut src| {
                src.set_volume(BGM_VOLUME);
                src.set_repeat(true);
                src.set_query_interval(Duration::ZERO);
                src
            })
            .map(|mut src| {
                match bgm {
                    Bgm::Title => {
                        src.set_fade_in(Duration::from_secs(2));
                        src
                    }
                    _ => src
                }
            });

        if let Some(mut src) = src {
            src.play_later();
            self.playing_src = Some(src);
        }
    }

    pub fn stop_bgm(&mut self) {
        self.playing_src = None
    }

    pub fn play_se(&self, ctx: &mut Context, se: Se) {
        let src = self.se_data_map.get(&se)
            .and_then(|data| audio::Source::from_data(ctx, data.clone()).ok())
            .map(|mut src| {
                src.set_volume(SE_VOLUME);
                src
            });

        if let Some(mut src) = src {
            src.play_detached(ctx);
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum Bgm {
    Title,
    InGame,
}

#[derive(Eq, PartialEq, Hash)]
pub enum Se {
    MenuClick,
    CountdownTick,
    GameStart,
}

pub struct Font {
    pub default: graphics::Font,
    pub vt323: graphics::Font,
}

impl Font {
    fn new(ctx: &mut Context) -> GameResult<Font> {
        Ok(
            Font {
                default: graphics::Font::new(ctx, "/font/Play-Regular.ttf")?,
                vt323: graphics::Font::new(ctx, "/font/VT323-Regular.ttf")?,
            }
        )
    }
}

pub struct Color {
    pub background: graphics::Color,
}

impl Color {
    fn new() -> Color {
        Color {
            background: graphics::Color::from_rgb(46, 46, 46),
        }
    }
}