use std::collections::HashMap;
use std::time::Duration;

use ggez::{audio, Context, GameResult, graphics};
use ggez::audio::SoundSource;

use crate::tetris::game::MinoBlock;

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
    uncolored_mino_block: graphics::Image,
    mino_block_images: HashMap<MinoBlock, graphics::Image>,
}

impl Image {
    fn new(ctx: &mut Context) -> GameResult<Image> {
        Ok(
            Image {
                cursor: graphics::Image::new(ctx, "/image/cursor.png")?,
                title_particle: graphics::Image::new(ctx, "/image/particles/title.png")?,
                uncolored_mino_block: graphics::Image::new(ctx, "/image/mino_block.png")?,
                mino_block_images: HashMap::new(),
            }
        )
    }

    pub fn mino_block<'a>(&'a mut self, ctx: &mut Context, block: &MinoBlock) -> GameResult<&'a graphics::Image> {
        if self.mino_block_images.get(block).is_none() {
            let _ = self.mino_block_images.insert(
                block.clone(),
                self.gen_colorized_mino_block(ctx, block,
                )?,
            );
        }

        Ok(self.mino_block_images.get(block).expect("failed to gen image"))
    }

    fn gen_colorized_mino_block(&self, ctx: &mut Context, block: &MinoBlock) -> GameResult<graphics::Image> {
        const RED_VALUE: usize = 0;
        const GREEN_VALUE: usize = 1;
        const BLUE_VALUE: usize = 2;
        const ALPHA_VALUE: usize = 3;

        let w = self.uncolored_mino_block.width();
        let h = self.uncolored_mino_block.height();

        let rgba = self.uncolored_mino_block
            .to_rgba8(ctx)?
            .iter()
            .enumerate()
            .map(|(idx, &v)| {
                match block {
                    MinoBlock::AQUA => match idx % 4 {
                        RED_VALUE => v.saturating_sub(64),
                        BLUE_VALUE | GREEN_VALUE => v.saturating_add(80),
                        ALPHA_VALUE => 255u8,
                        _ => unreachable!(),
                    },
                    MinoBlock::YELLOW => match idx % 4 {
                        RED_VALUE | GREEN_VALUE => v.saturating_add(80),
                        BLUE_VALUE => v.saturating_sub(64),
                        ALPHA_VALUE => 255u8,
                        _ => unreachable!(),
                    },
                    MinoBlock::PURPLE => match idx % 4 {
                        RED_VALUE | BLUE_VALUE => v.saturating_add(80),
                        GREEN_VALUE => v.saturating_sub(64),
                        ALPHA_VALUE => 255u8,
                        _ => unreachable!(),
                    },
                    MinoBlock::BLUE => match idx % 4 {
                        BLUE_VALUE => v.saturating_add(80),
                        RED_VALUE | GREEN_VALUE => v.saturating_sub(64),
                        ALPHA_VALUE => 255u8,
                        _ => unreachable!(),
                    },
                    MinoBlock::ORANGE => match idx % 4 {
                        RED_VALUE => v.saturating_add(172),
                        GREEN_VALUE => v.saturating_add(48),
                        BLUE_VALUE => v.saturating_sub(48),
                        ALPHA_VALUE => 255u8,
                        _ => unreachable!(),
                    },
                    MinoBlock::GREEN => match idx % 4 {
                        GREEN_VALUE => v.saturating_add(80),
                        RED_VALUE | BLUE_VALUE => v.saturating_sub(64),
                        ALPHA_VALUE => 255u8,
                        _ => unreachable!(),
                    },
                    MinoBlock::RED => match idx % 4 {
                        RED_VALUE => v.saturating_add(80),
                        GREEN_VALUE | BLUE_VALUE => v.saturating_sub(64),
                        ALPHA_VALUE => 255u8,
                        _ => unreachable!(),
                    },
                }
            })
            .collect::<Vec<_>>();

        Ok(graphics::Image::from_rgba8(ctx, w, h, rgba.as_slice())?)
    }
}

pub struct Audio {
    bgm_data_map: HashMap<Bgm, audio::SoundData>,
    se_data_map: HashMap<Se, audio::SoundData>,

    #[allow(dead_code)]
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
                Se::MinoMove => "/sound/se/thinkpad_shift_click.mp3",
                Se::MinoSpin => "/sound/se/se_maoudamashii_se_pc03.mp3",
                Se::MinoDropSoftly => "/sound/se/hhkb_fn_click.mp3",
                Se::MinoDropHardly => "/sound/se/hhkb_fn_click.mp3",
            }
        }
        let se_data_map = maplit::hashmap! {
            Se::MenuClick => audio::SoundData::new(ctx, se_path(Se::MenuClick))?,
            Se::CountdownTick => audio::SoundData::new(ctx, se_path(Se::CountdownTick))?,
            Se::GameStart => audio::SoundData::new(ctx, se_path(Se::GameStart))?,
            Se::MinoMove => audio::SoundData::new(ctx, se_path(Se::MinoMove))?,
            Se::MinoSpin => audio::SoundData::new(ctx, se_path(Se::MinoSpin))?,
            Se::MinoDropSoftly => audio::SoundData::new(ctx, se_path(Se::MinoDropSoftly))?,
            Se::MinoDropHardly => audio::SoundData::new(ctx, se_path(Se::MinoDropHardly))?,
        };

        Ok(
            Audio {
                bgm_data_map,
                se_data_map,
                playing_src: None,
            }
        )
    }

    pub fn play_bgm(&mut self, ctx: &mut Context, bgm: Bgm) -> GameResult {
        self.stop_bgm();

        let src = self.bgm_data_map.get(&bgm)
            .and_then(|data| audio::Source::from_data(ctx, data.clone()).ok())
            .map(|mut src| {
                src.set_repeat(true);
                src.set_query_interval(Duration::ZERO);
                src
            })
            .map(|mut src| {
                match bgm {
                    Bgm::Title => {
                        src.set_volume(0.15);
                        src.set_fade_in(Duration::from_secs(2));
                        src
                    }
                    Bgm::InGame => {
                        src.set_volume(0.12);
                        src.set_pitch(0.9);
                        src
                    }
                }
            });

        if let Some(src) = src {
            src.play_later()?;
            self.playing_src = Some(src);
        }

        Ok(())
    }

    pub fn stop_bgm(&mut self) {
        self.playing_src = None
    }

    pub fn play_se(&self, ctx: &mut Context, se: Se) -> GameResult {
        let src = self.se_data_map.get(&se)
            .and_then(|data| audio::Source::from_data(ctx, data.clone()).ok())
            .map(|mut src| {
                match se {
                    Se::MenuClick => {
                        src.set_volume(0.15);
                    }
                    Se::GameStart => {
                        src.set_volume(0.4);
                    }
                    Se::CountdownTick => {
                        src.set_volume(0.45);
                    }
                    Se::MinoMove => {
                        src.set_volume(0.8);
                        src.set_pitch(0.6);
                    }
                    Se::MinoSpin => {
                        src.set_volume(0.75);
                    }
                    Se::MinoDropSoftly => {
                        src.set_volume(0.15);
                    },
                    Se::MinoDropHardly => {
                        src.set_volume(0.6);
                        src.set_pitch(0.65);
                    }
                }
                src
            });

        if let Some(mut src) = src {
            src.play_detached(ctx)?;
        }

        Ok(())
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
    MinoMove,
    MinoSpin,
    MinoDropSoftly,
    MinoDropHardly,
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
    pub panel: graphics::Color,
    pub separator: graphics::Color,
    pub grid_line: graphics::Color,
}

impl Color {
    fn new() -> Color {
        Color {
            background: graphics::Color::from_rgb(24, 24, 24),
            panel: graphics::Color::from_rgba(48, 240, 255, 16),
            separator: graphics::Color::from_rgb(64, 64, 64),
            grid_line: graphics::Color::from_rgba(24, 24, 24, 128),
        }
    }
}