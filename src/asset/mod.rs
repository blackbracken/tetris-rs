use ggez::{Context, GameResult};

use crate::asset::audio::Audio;
use crate::asset::color::Color;
use crate::asset::font::Font;
use crate::asset::image::Image;

pub mod audio;
pub mod color;
pub mod font;
pub mod image;

pub struct Asset {
    pub image: Image,
    pub audio: Audio,
    pub font: Font,
    pub color: Color,
}

impl Asset {
    pub fn load(ctx: &mut Context) -> GameResult<Box<Asset>> {
        Ok(Box::new(Asset {
            image: Image::new(ctx)?,
            audio: Audio::new(ctx)?,
            font: Font::new(ctx)?,
            color: Color::new(),
        }))
    }
}
