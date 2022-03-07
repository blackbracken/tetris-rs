use super::mino::{block::Block, spin::MinoRotation, Mino};
use crate::kernel::xy_pos::I16XYPos;

const FIELD_UNIT_WIDTH: usize = 10;
const FIELD_UNIT_HEIGHT: usize = 20; // NOTE: 21行目以降を表示しないので20で済ませている

/// 盤面の状態を表現する.
pub struct Playfield {
    /// 盤面上で配置が確定したミノブロック
    pub confirmed: [[Option<Block>; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT],

    pub dropping: Mino,
    pub dropping_pos: I16XYPos,
    pub dropping_rot: MinoRotation,
}

impl Playfield {
    pub fn new(dropping: Mino) -> Playfield {
        let confirmed = [[None; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT];

        Playfield {
            confirmed,
            dropping,
            dropping_pos: (4_i16, 1_i16).into(),
            dropping_rot: MinoRotation::Clockwise,
        }
    }
}
