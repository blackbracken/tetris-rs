use std::collections::HashMap;

use ggez::{graphics, Context, GameResult};

use crate::tetris::model::tetrimino::MinoBlock;

pub struct Image {
    pub cursor: graphics::Image,
    pub title_particle: graphics::Image,
    pub dropping_windbreak_particle: graphics::Image,
    uncolored_mino_block: graphics::Image,
    mino_block_images: HashMap<MinoBlock, graphics::Image>,
}

impl Image {
    pub(super) fn new(ctx: &mut Context) -> GameResult<Image> {
        Ok(Image {
            cursor: graphics::Image::new(ctx, "/image/cursor.png")?,
            title_particle: graphics::Image::new(ctx, "/image/particles/title.png")?,
            uncolored_mino_block: graphics::Image::new(ctx, "/image/mino_block.png")?,
            dropping_windbreak_particle: graphics::Image::new(
                ctx,
                "/image/particles/dropping_windbreak.png",
            )?,
            mino_block_images: HashMap::new(),
        })
    }

    pub fn mino_block<'a>(
        &'a mut self,
        ctx: &mut Context,
        block: &MinoBlock,
    ) -> GameResult<&'a graphics::Image> {
        if self.mino_block_images.get(block).is_none() {
            let _ = self
                .mino_block_images
                .insert(block.to_owned(), self.gen_colorized_mino_block(ctx, block)?);
        }

        Ok(self
            .mino_block_images
            .get(block)
            .expect("failed to gen image"))
    }

    fn gen_colorized_mino_block(
        &self,
        ctx: &mut Context,
        block: &MinoBlock,
    ) -> GameResult<graphics::Image> {
        const RED_VALUE: usize = 0;
        const GREEN_VALUE: usize = 1;
        const BLUE_VALUE: usize = 2;
        const ALPHA_VALUE: usize = 3;

        let w = self.uncolored_mino_block.width();
        let h = self.uncolored_mino_block.height();

        let rgba = self
            .uncolored_mino_block
            .to_rgba8(ctx)?
            .iter()
            .enumerate()
            .map(|(idx, &v)| match block {
                MinoBlock::AQUA => match idx % 4 {
                    RED_VALUE => v.saturating_sub(64),
                    BLUE_VALUE | GREEN_VALUE => v.saturating_add(80),
                    ALPHA_VALUE => 255u8,
                    _ => unreachable!(),
                },
                MinoBlock::YELLOW => match idx % 4 {
                    RED_VALUE | GREEN_VALUE => v.saturating_add(80),
                    BLUE_VALUE => v.saturating_sub(64),
                    ALPHA_VALUE => 255u8,
                    _ => unreachable!(),
                },
                MinoBlock::PURPLE => match idx % 4 {
                    RED_VALUE | BLUE_VALUE => v.saturating_add(80),
                    GREEN_VALUE => v.saturating_sub(64),
                    ALPHA_VALUE => 255u8,
                    _ => unreachable!(),
                },
                MinoBlock::BLUE => match idx % 4 {
                    BLUE_VALUE => v.saturating_add(80),
                    RED_VALUE | GREEN_VALUE => v.saturating_sub(64),
                    ALPHA_VALUE => 255u8,
                    _ => unreachable!(),
                },
                MinoBlock::ORANGE => match idx % 4 {
                    RED_VALUE => v.saturating_add(172),
                    GREEN_VALUE => v.saturating_add(48),
                    BLUE_VALUE => v.saturating_sub(48),
                    ALPHA_VALUE => 255u8,
                    _ => unreachable!(),
                },
                MinoBlock::GREEN => match idx % 4 {
                    GREEN_VALUE => v.saturating_add(80),
                    RED_VALUE | BLUE_VALUE => v.saturating_sub(64),
                    ALPHA_VALUE => 255u8,
                    _ => unreachable!(),
                },
                MinoBlock::RED => match idx % 4 {
                    RED_VALUE => v.saturating_add(80),
                    GREEN_VALUE | BLUE_VALUE => v.saturating_sub(64),
                    ALPHA_VALUE => 255u8,
                    _ => unreachable!(),
                },
            })
            .collect::<Vec<_>>();

        Ok(graphics::Image::from_rgba8(ctx, w, h, rgba.as_slice())?)
    }
}
