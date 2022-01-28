use ggez::{graphics::Image, Context, GameResult};

pub struct ImagePath<'a>(pub &'a str);

pub const IMG_CURSOR: ImagePath = ImagePath("/image/cursor.png");
pub const IMG_TITLE_PARTICLE: ImagePath = ImagePath("/image/particles/title.png");
pub const IMG_DROPPING_WINDBREAK_PARTICLE: ImagePath =
    ImagePath("/image/particles/dropping_windbreak.png");

pub trait AssetProvider {
    fn image(&mut self, ctx: &mut Context, path: ImagePath) -> GameResult<&Image>;
}
