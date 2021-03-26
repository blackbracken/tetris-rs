use std::collections::HashMap;
use std::time::Duration;

use ggez::{audio, Context, GameResult, graphics};
use ggez::audio::SoundSource;

const BGM_VOLUME: f32 = 0.2;
const SE_VOLUME: f32 = 0.1;

pub struct Asset {
    pub image: Image,
    pub audio: Audio,
    pub font: Font,
    pub color: Color,
}

impl Asset {
    // TODO: support asynchronous loading
    pub fn load(ctx: &mut Context) -> GameResult<Box<Asset>> {
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
}

impl Image {
    fn new(ctx: &mut Context) -> GameResult<Image> {
        Ok(
            Image {
                cursor: graphics::Image::new(ctx, "/cursor.png")?,
                title_particle: graphics::Image::new(ctx, "/particles/title.png")?,
                mino_block: graphics::Image::new(ctx, "/block.png")?,
            }
        )
    }

    pub fn colored_block(&self, ctx: &mut Context) -> GameResult<graphics::Image> {
        let w = self.mino_block.width();
        let h = self.mino_block.height();
        let colored_block: Vec<u8> = self.mino_block
            .to_rgba8(ctx)?
            .iter()
            .enumerate()
            .map(|(idx, v)| match idx % 4 {
                0 => v.saturating_add(64),
                1 | 2 => v.saturating_sub(64),
                3 => 255u8,
                _ => *v,
            })
            .collect();

        graphics::Image::from_rgba8(ctx, w, h, &colored_block)
    }
}


pub struct Audio {
    bgm_data_map: HashMap<Bgm, audio::SoundData>,
    se_data_map: HashMap<Se, audio::SoundData>,

    playing_src: Option<audio::Source>,
}

impl Audio {
    fn new(ctx: &mut Context) -> GameResult<Audio> {
        let bgm_data_map = maplit::hashmap! {
            Bgm::Title => audio::SoundData::new(ctx, "/sound/bgm_maoudamashii_cyber18.mp3")?,
        };

        let se_data_map = maplit::hashmap! {
            Se::MenuClick => audio::SoundData::new(ctx, "/sound/se_maoudamashii_system26.mp3")?,
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

        if let Some(src) = src {
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
    None,
}

#[derive(Eq, PartialEq, Hash)]
pub enum Se {
    MenuClick,
}

pub struct Font {
    pub default: graphics::Font,
}

impl Font {
    fn new(ctx: &mut Context) -> GameResult<Font> {
        Ok(
            Font {
                default: graphics::Font::new(ctx, "/Play-Regular.ttf")?,
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