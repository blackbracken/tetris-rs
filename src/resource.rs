use std::time::Duration;

use ggez::{audio, Context, GameResult, graphics};
use ggez::audio::SoundSource;

pub struct SharedResource {
    pub default_font: graphics::Font,
    pub title_music: audio::Source,
    pub cursor_image: graphics::Image,
    pub title_particle_image: graphics::Image,
    pub block_image: graphics::Image,
    pub red_block_image: graphics::Image,
    pub background_color: graphics::Color,
}

impl SharedResource {
    // TODO: support asynchronous loading
    pub fn load(ctx: &mut Context) -> GameResult<Box<SharedResource>> {
        let play_regular_font = graphics::Font::new(ctx, "/Play-Regular.ttf")?;

        let mut title_music = audio::Source::new(ctx, "/music/bgm_maoudamashii_cyber18.mp3")?;
        title_music.set_volume(0.2);
        title_music.set_repeat(true);
        title_music.set_fade_in(Duration::from_secs(2));

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


        Ok(
            Box::new(
                SharedResource {
                    default_font: play_regular_font,
                    title_music,
                    title_particle_image,
                    cursor_image,
                    block_image,
                    red_block_image,
                    background_color: graphics::Color::from_rgb(46, 46, 46),
                }
            )
        )
    }
}