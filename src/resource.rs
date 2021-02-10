use ggez::{Context, GameResult, graphics};

pub struct SharedResource {
    pub default_font: graphics::Font,
    pub cursor_image: graphics::Image,
    pub background_color: graphics::Color,
}

impl SharedResource {
    // TODO: support asynchronous loading
    pub fn load(ctx: &mut Context) -> GameResult<Box<SharedResource>> {
        let play_regular_font = graphics::Font::new(ctx, "/Play-Regular.ttf")?;
        let cursor_image = graphics::Image::new(ctx, "/cursor.png")?;

        Ok(
            Box::new(
                SharedResource {
                    default_font: play_regular_font,
                    cursor_image,
                    background_color: graphics::Color::from_rgb(46, 46, 46),
                }
            )
        )
    }
}