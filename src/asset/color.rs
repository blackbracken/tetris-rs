use ggez::graphics;

use crate::tetris::model::tetrimino::MinoBlock;

pub struct Color {
    pub background: graphics::Color,
    pub panel: graphics::Color,
    pub separator: graphics::Color,
    pub grid_line: graphics::Color,
    pub frame: graphics::Color,
}

impl Color {
    pub(super) fn new() -> Color {
        Color {
            background: graphics::Color::from_rgb(24, 24, 24),
            panel: graphics::Color::from_rgba(128, 255, 255, 32),
            separator: graphics::Color::from_rgb(64, 64, 64),
            grid_line: graphics::Color::from_rgba(24, 24, 24, 128),
            frame: graphics::Color::from_rgb(148, 148, 148),
        }
    }

    pub fn block(block: &MinoBlock) -> graphics::Color {
        match block {
            MinoBlock::AQUA => graphics::Color::from_rgb(32, 184, 184),
            MinoBlock::YELLOW => graphics::Color::from_rgb(184, 184, 32),
            MinoBlock::PURPLE => graphics::Color::from_rgb(184, 32, 184),
            MinoBlock::BLUE => graphics::Color::from_rgb(32, 32, 184),
            MinoBlock::ORANGE => graphics::Color::from_rgb(255, 148, 64),
            MinoBlock::GREEN => graphics::Color::from_rgb(32, 184, 32),
            MinoBlock::RED => graphics::Color::from_rgb(184, 32, 32),
        }
    }
}