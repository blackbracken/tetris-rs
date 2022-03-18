use derive_new::new;

use super::mino::{block::Block, spin::MinoRotation, Mino, WhichMold};
use crate::kernel::xy_pos::I16XYPos;

const FIELD_UNIT_WIDTH: usize = 10;
const FIELD_UNIT_HEIGHT: usize = 20; // NOTE: 21行目以降を表示しないので20で済ませている

#[derive(Clone)]
struct Board([[Option<Block>; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT]);

impl Board {
    pub fn demonstrated(mut self, drop: Drop) -> Self {
        let mold = drop.mino.mold;

        fn f<const N: usize>() {
            for (y, line) in mold.matrix.iter().enumerate() {
                for (x, exists) in line.iter().enumerate() {
                    board[y][x] = Some(self.dropping.block).filter(|_| *exists);
                }
            }
        }

        self
    }


}

#[derive(new)]
struct Drop {
    pub mino: Mino,
    pub pos: I16XYPos,
    pub rot: MinoRotation,
}

/// 盤面の状態を表現する.
pub struct Playfield {
    /// 盤面上で配置が確定したミノブロック
    pub confirmed: Board,
    pub drop: Drop,
}

impl Playfield {
    pub fn new(dropping: Mino) -> Playfield {
        let confirmed = Board([[None; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT]);

        Playfield {
            confirmed,
            drop: Drop::new(dropping, (4_i16, 1_i16).into(), MinoRotation::Clockwise),
        }
    }

    pub fn board(&self) -> Board {
        let mut board = self.confirmed;

        match self.dropping.mold {
            WhichMold::Square2(mold) => {
                for (y, line) in mold.matrix.iter().enumerate() {
                    for (x, exists) in line.iter().enumerate() {
                        board[y][x] = Some(self.dropping.block).filter(|_| *exists);
                    }
                }
            }
            WhichMold::Square3(mold) => {
                for (y, line) in mold.matrix.iter().enumerate() {
                    for (x, exists) in line.iter().enumerate() {
                        board[y][x] = Some(self.dropping.block).filter(|_| *exists);
                    }
                }
            }
            WhichMold::Square4(mold) => {
                for (y, line) in mold.matrix.iter().enumerate() {
                    for (x, exists) in line.iter().enumerate() {
                        board[y][x] = Some(self.dropping.block).filter(|_| *exists);
                    }
                }
            }
        }

        board
    }
}
