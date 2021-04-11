use ggez::{Context, GameResult, graphics};

pub struct Font {
    pub default: graphics::Font,
    pub vt323: graphics::Font,
}

impl Font {
    pub(super) fn new(ctx: &mut Context) -> GameResult<Font> {
        Ok(
            Font {
                default: graphics::Font::new(ctx, "/font/Play-Regular.ttf")?,
                vt323: graphics::Font::new(ctx, "/font/VT323-Regular.ttf")?,
            }
        )
    }
}