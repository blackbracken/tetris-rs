use crate::kernel::{tetris::mino::mold::MinoMold, xy_pos::I16XYPos};

pub mod block;
pub mod mold;
pub mod spin;

pub type WallkickOffset = I16XYPos;

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

pub const MINO_T: Mino = Mino::T {
    mold: todo!(),
    wallkick_offset: todo!(),
};

pub enum Mino {
    T {
        mold: MinoMold<3>,
        wallkick_offset: WallkickOffset,
    },
}
