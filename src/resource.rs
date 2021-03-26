use std::collections::HashMap;
use std::time::Duration;

use ggez::{audio, Context, GameResult, graphics};
use ggez::audio::SoundSource;

const BGM_VOLUME: f32 = 0.2;
const SE_VOLUME: f32 = 0.1;

pub struct SharedResource {
    pub default_font: graphics::Font,
    pub cursor_image: graphics::Image,
    pub title_particle_image: graphics::Image,
    pub block_image: graphics::Image,
    pub red_block_image: graphics::Image,
    pub background_color: graphics::Color,
    bgm_player: BgmPlayer,
    bgm_data_map: HashMap<Bgm, audio::SoundData>,
    se_data_map: HashMap<Se, audio::SoundData>,
}

impl SharedResource {
    // TODO: support asynchronous loading
    pub fn load(ctx: &mut Context) -> GameResult<Box<SharedResource>> {
        let play_regular_font = graphics::Font::new(ctx, "/Play-Regular.ttf")?;

        let cursor_image = graphics::Image::new(ctx, "/cursor.png")?;

        let title_particle_image = graphics::Image::new(ctx, "/particles/title.png")?;

        let block_image = graphics::Image::new(ctx, "/block.png")?;

        let x: Vec<u8> = block_image
            .to_rgba8(ctx)?
            .iter()
            .enumerate()
            .map(|(idx, value)| match idx % 4 {
                0 => value.saturating_add(64),
                1 | 2 => value.saturating_sub(64),
                3 => 255u8,
                _ => *value,
            })
            .collect();
        let red_block_image = graphics::Image::from_rgba8(ctx, block_image.height(), block_image.width(), x.as_ref())?;

        let bgm_data_map = maplit::hashmap! {
            Bgm::Title => audio::SoundData::new(ctx, "/sound/bgm_maoudamashii_cyber18.mp3")?,
        };

        let se_data_map = maplit::hashmap! {
            Se::MenuClick => audio::SoundData::new(ctx, "/sound/se_maoudamashii_system26.mp3")?,
        };

        Ok(
            Box::new(
                SharedResource {
                    default_font: play_regular_font,
                    title_particle_image,
                    cursor_image,
                    block_image,
                    red_block_image,
                    background_color: graphics::Color::from_rgb(46, 46, 46),
                    bgm_player: BgmPlayer::new(),
                    bgm_data_map,
                    se_data_map,
                }
            )
        )
    }

    pub fn play_bgm(&mut self, ctx: &mut Context, bgm: Bgm) {
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
            self.bgm_player.play(ctx, src);
        } else {
            self.bgm_player.stop(ctx);
        }
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

struct BgmPlayer {
    playing_src: Option<audio::Source>,
}

impl BgmPlayer {
    fn new() -> BgmPlayer {
        BgmPlayer {
            playing_src: None,
        }
    }

    pub fn play(&mut self, ctx: &mut Context, src: audio::Source) {
        self.stop(ctx);

        src.play_later();
        self.playing_src = Some(src);
    }

    pub fn stop(&mut self, ctx: &mut Context) {
        if let Some(ref mut p) = self.playing_src {
            p.stop(ctx);
        }
        self.playing_src = None;
    }
}