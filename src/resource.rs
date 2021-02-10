use ggez::{Context, GameResult, graphics};

pub struct Resource {
    pub default_font: graphics::Font,
    pub cursor_image: graphics::Image,
}

impl Resource {
    // TODO: support asynchronous loading
    pub fn load(ctx: &mut Context) -> GameResult<Resource> {
        let play_regular_font = graphics::Font::new(ctx, "/Play-Regular.ttf")?;
        let cursor_image = graphics::Image::new(ctx, "/cursor.png")?;

        Ok(
            Resource {
                default_font: play_regular_font,
                cursor_image,
            }
        )
    }
}