use ggez::{graphics::Image, Context, GameResult};

pub struct ImagePath<'a>(pub &'a str);

pub trait AssetProvider {
    fn image(&mut self, ctx: &mut Context, path: ImagePath) -> GameResult<&Image>;
}
