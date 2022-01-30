use ggez::{graphics, Context, GameResult};

pub struct Image {
    pub cursor: graphics::Image,
    pub title_particle: graphics::Image,
    pub dropping_windbreak_particle: graphics::Image,
    uncolored_mino_block: graphics::Image,
}

impl Image {
    pub(super) fn new(ctx: &mut Context) -> GameResult<Image> {
        Ok(Image {
            cursor: graphics::Image::new(ctx, "/image/cursor.png")?,
            title_particle: graphics::Image::new(ctx, "/image/particles/title.png")?,
            uncolored_mino_block: graphics::Image::new(ctx, "/image/mino_block/black.png")?,
            dropping_windbreak_particle: graphics::Image::new(
                ctx,
                "/image/particles/dropping_windbreak.png",
            )?,
        })
    }
}
