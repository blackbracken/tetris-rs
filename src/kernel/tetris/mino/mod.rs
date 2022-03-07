use once_cell::sync::Lazy;

use crate::kernel::{tetris::mino::mold::MinoMold, xy_pos::I16XYPos};

pub mod block;
pub mod mold;
pub mod spin;

pub type WallkickOffsets = [I16XYPos; 5];

// TODO: 移動する?
/// テトリスのミノの色を表現する列挙.
pub enum Color {
    AQUA,
    YELLOW,
    PURPLE,
    BLUE,
    ORANGE,
    GREEN,
    RED,
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

static MINO_T: Lazy<Mino> = Lazy::new(|| Mino::T {
    #[rustfmt::skip]
    mold: MinoMold::square_n([
        [0, 1, 0],
        [1, 1, 1],
        [0, 0, 0]
    ]),
});

#[derive(Clone, Copy)]
pub enum Mino {
    // NOTE: moldは const generics により静的にサイズが決定されるためにフィールドとなっている
    T { mold: MinoMold<3> },
}

impl Mino {
    pub fn all() -> Vec<Mino> {
        vec![*MINO_T]
    }

    pub fn wallkick_offsets(&self) -> WallkickOffsets {
        match self {
            Mino::T { .. } => [
                (0, 0).into(),
                (1, 0).into(),
                (1, 1).into(),
                (0, -2).into(),
                (1, -2).into(),
            ],
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Mino::T { .. } => Color::PURPLE,
        }
    }
}
