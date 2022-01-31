use ggez::{
    graphics::{Font, Image},
    Context,
    GameResult,
};

pub struct ImagePath(pub &'static str);

pub struct FontPath(pub &'static str);

pub const IMG_CURSOR: ImagePath = ImagePath("/image/cursor.png");
pub const IMG_TITLE_PARTICLE: ImagePath = ImagePath("/image/particles/title.png");
pub const IMG_DROPPING_WINDBREAK_PARTICLE: ImagePath =
    ImagePath("/image/particles/dropping_windbreak.png");
pub const IMG_AQUA_MINO_BLOCK: ImagePath = ImagePath("/image/mino_block/aqua.png");
pub const IMG_BLUE_MINO_BLOCK: ImagePath = ImagePath("/image/mino_block/blue.png");
pub const IMG_GREEN_MINO_BLOCK: ImagePath = ImagePath("/image/mino_block/green.png");
pub const IMG_ORANGE_MINO_BLOCK: ImagePath = ImagePath("/image/mino_block/orange.png");
pub const IMG_PINK_MINO_BLOCK: ImagePath = ImagePath("/image/mino_block/pink.png");
pub const IMG_RED_MINO_BLOCK: ImagePath = ImagePath("/image/mino_block/red.png");
pub const IMG_YELLOW_MINO_BLOCK: ImagePath = ImagePath("/image/mino_block/yellow.png");

pub const FONT_PLAY: FontPath = FontPath("/font/Play-Regular.ttf");
pub const FONT_VT323: FontPath = FontPath("/font/VT323-Regular.ttf");

pub trait AssetProvider {
    fn image(&mut self, ctx: &mut Context, path: ImagePath) -> GameResult<&Image>;
    fn font(&mut self, ctx: &mut Context, path: FontPath) -> GameResult<&Font>;
}
