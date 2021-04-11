use crate::tetris::model::tetrimino::MinoBlock;

/// # ミノエンティティ
///
/// フィールド上のマスにあるものを表現する
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum MinoEntity {
    AQUA,
    YELLOW,
    PURPLE,
    BLUE,
    ORANGE,
    GREEN,
    RED,

    AIR,
}

impl MinoEntity {
    pub fn block(&self) -> Option<MinoBlock> {
        use MinoBlock::*;

        match self {
            MinoEntity::AQUA => Some(AQUA),
            MinoEntity::YELLOW => Some(YELLOW),
            MinoEntity::PURPLE => Some(PURPLE),
            MinoEntity::BLUE => Some(BLUE),
            MinoEntity::ORANGE => Some(ORANGE),
            MinoEntity::GREEN => Some(GREEN),
            MinoEntity::RED => Some(RED),
            MinoEntity::AIR => None,
        }
    }

    pub fn is_air(&self) -> bool {
        self == &MinoEntity::AIR
    }
}

impl Into<MinoEntity> for MinoBlock {
    fn into(self) -> MinoEntity {
        use MinoEntity::*;

        match self {
            MinoBlock::AQUA => AQUA,
            MinoBlock::YELLOW => YELLOW,
            MinoBlock::PURPLE => PURPLE,
            MinoBlock::BLUE => BLUE,
            MinoBlock::ORANGE => ORANGE,
            MinoBlock::GREEN => GREEN,
            MinoBlock::RED => RED,
        }
    }
}
