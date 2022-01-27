use ggez::{graphics::Image, Context, GameResult};

pub trait AssetProvider {
    fn image(&mut self, ctx: &mut Context, path: &str) -> GameResult<&Image>;
}
