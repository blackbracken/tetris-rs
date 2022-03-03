use ggez::{
    graphics::{Font, Image},
    Context,
    GameResult,
};

pub struct ImagePath(pub &'static str);

pub struct FontPath(pub &'static str);

pub trait AssetProvider {
    fn image(&mut self, ctx: &mut Context, path: ImagePath) -> GameResult<&Image>;
    fn font(&mut self, ctx: &mut Context, path: FontPath) -> GameResult<&Font>;
}
