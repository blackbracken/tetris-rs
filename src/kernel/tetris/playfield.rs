use std::{convert::From, fmt};

use derive_new::new;

use super::mino::{block::Block, spin::MinoRotation, Mino};
use crate::kernel::xy_pos::I16XYPos;

const FIELD_UNIT_WIDTH: usize = 10;
const FIELD_UNIT_HEIGHT: usize = 20; // NOTE: 21行目以降を表示しないので20で済ませている

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board([[Option<Block>; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT]);

impl Board {
    fn blank() -> Self {
        Board([[None; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT])
    }

    fn demonstrated(&self, drop: Drop) -> Self {
        let mut board_mat = self.0.clone();
        let drop_mat = drop.mino.mold.matrix();

        for (y, line) in drop_mat.iter().enumerate() {
            for (x, exists) in line.iter().enumerate() {
                let (dx, dy) = drop.pos.into();
                let x = x + dx as usize;
                let y = y + dy as usize;

                if let (0..10, 0..20) = (x, y) {
                    board_mat[y][x] = Some(drop.mino.block).filter(|_| *exists);
                }
            }
        }

        Board(board_mat)
    }
}

#[derive(new, Clone)]
pub struct Drop {
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
        let confirmed = Board::blank();

        Playfield {
            confirmed,
            drop: Drop::new(dropping, (4_i16, 1_i16).into(), MinoRotation::Clockwise),
        }
    }

    pub fn board(&self) -> Board {
        self.confirmed.demonstrated(self.drop.clone())
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;
    use crate::kernel::tetris::mino::MINO_T;

    #[test]
    fn test_demonstrated_blank() {
        let actual = Board::blank();

        let expected_mat = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let expected = make_monocolor_board(Block::PURPLE, expected_mat);

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_demonstrated_initial() {
        let board = Board::blank();
        let drop = Drop::new(*MINO_T, (3, 1).into(), MinoRotation::Clockwise);

        let actual = board.demonstrated(drop);

        let expected_mat = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let expected = make_monocolor_board(Block::PURPLE, expected_mat);

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_demonstrated_out_of_board_partially() {
        let board = Board::blank();
        let drop = Drop::new(*MINO_T, (8, 1).into(), MinoRotation::Clockwise);

        let actual = board.demonstrated(drop);

        let expected_mat = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let expected = make_monocolor_board(Block::PURPLE, expected_mat);

        assert_eq!(actual, expected)
    }

    fn make_monocolor_board(
        block: Block,
        matrix: [[usize; FIELD_UNIT_WIDTH]; FIELD_UNIT_HEIGHT],
    ) -> Board {
        let matrix = matrix.map(|line| line.map(|n| Some(block).filter(|_| n != 0)));

        Board(matrix)
    }
}
