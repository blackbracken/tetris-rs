use crate::kernel::{tetris::mino_mold::MinoMold, xy_pos::I16XYPos};

/*
   T,
   S,
   Z,
   L,
   J,
   O,
   I,
*/

pub struct Mino<const S: usize> {
    pub mold: MinoMold<S>,
}
