use derive_new::new;
use once_cell::sync::Lazy;

use self::block::Block;
use crate::kernel::{tetris::mino::mold::MinoMold, xy_pos::I16XYPos};

pub mod block;
pub mod mold;
pub mod spin;

pub type WallkickOffsets = [I16XYPos; 5];

#[derive(Clone, Copy)]
pub enum WhichMold {
    Square2(MinoMold<2>),
    Square3(MinoMold),
    Square4(MinoMold<4>),
}

impl WhichMold {
    pub fn matrix(&self) -> Vec<Vec<bool>> {
        match self {
            WhichMold::Square2(mold) => mold
                .matrix
                .iter()
                .map(|line| line.iter().map(|b| *b).collect())
                .collect(),
            WhichMold::Square3(mold) => mold
                .matrix
                .iter()
                .map(|line| line.iter().map(|b| *b).collect())
                .collect(),
            WhichMold::Square4(mold) => mold
                .matrix
                .iter()
                .map(|line| line.iter().map(|b| *b).collect())
                .collect(),
        }
    }

    fn square_2(matrix: [[u8; 2]; 2]) -> WhichMold {
        WhichMold::Square2(MinoMold::square_n(matrix))
    }

    fn square_3(matrix: [[u8; 3]; 3]) -> WhichMold {
        WhichMold::Square3(MinoMold::square_n(matrix))
    }

    fn square_4(matrix: [[u8; 4]; 4]) -> WhichMold {
        WhichMold::Square4(MinoMold::square_n(matrix))
    }
}

/*
 TODO:
   T,
   S,
   Z,
   L,
   J,
   O,
   I,
*/

pub static MINO_T: Lazy<Mino> = Lazy::new(|| {
    Mino::new(
        #[rustfmt::skip]
        WhichMold::square_3([
            [0, 1, 0],
            [1, 1, 1],
            [0, 0, 0]
        ]),
        [
            (0, 0).into(),
            (1, 0).into(),
            (1, 1).into(),
            (0, -2).into(),
            (1, -2).into(),
        ],
        Block::PURPLE,
    )
});

#[derive(new, Clone, Copy)]
pub struct Mino {
    pub mold: WhichMold,
    pub wallkick_offsets: WallkickOffsets,
    pub block: Block,
}

pub fn all() -> Vec<Mino> {
    vec![*MINO_T]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_enumerates_whole_minos() {
        assert_eq!(all().len(), 1)
    }
}
