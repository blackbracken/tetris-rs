use derive_new::new;

use super::{
    board::Board,
    mino::{spin::MinoRotation, Mino},
};
use crate::kernel::xy_pos::I16XYPos;

#[derive(new, Clone)]
pub struct FieldDrop {
    pub mino: Mino,
    pub pos: I16XYPos,
    pub rot: MinoRotation,
}

/// 盤面の状態を表現する.
pub struct Playfield {
    /// 盤面上で配置が確定したミノブロック
    pub confirmed: Board,
    pub drop: FieldDrop,
}

impl Playfield {
    pub fn new(dropping: Mino) -> Playfield {
        let confirmed = Board::blank();

        Playfield {
            confirmed,
            drop: FieldDrop::new(dropping, (4_i16, 1_i16).into(), MinoRotation::Clockwise),
        }
    }

    pub fn board(&self) -> Board {
        self.confirmed.demonstrated(self.drop.clone())
    }
}
