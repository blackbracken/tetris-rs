use crate::kernel::{tetris::mino::mold::MinoMold, xy_pos::I16XYPos};

pub mod mold;
pub mod spin;

/*
   T,
   S,
   Z,
   L,
   J,
   O,
   I,
*/

pub type WallkickOffset = I16XYPos;

pub struct Mino<const S: usize> {
    pub mold: MinoMold<S>,
    pub wallkick_offset: WallkickOffset,
}
