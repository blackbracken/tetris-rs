use std::collections::VecDeque;

use rand::prelude::SliceRandom;

use crate::tetris::board::{Board, DroppingMinoStatus, MinoEntity};
use crate::tetris::tetrimino::Tetrimino;

pub struct Game {
    pub board: Board,
    pub bag: MinoBag,
    pub hold_mino: Option<Tetrimino>,
    pub did_already_hold: bool,
}

impl Game {
    pub fn new() -> Game {
        let mut bag = MinoBag::new();
        let dropping = bag.pop();

        Game {
            board: Board::new(dropping),
            bag,
            hold_mino: None,
            did_already_hold: false,
        }
    }

    pub fn move_left(&mut self) -> bool {
        self.board.try_move_x(-1)
    }

    pub fn move_right(&mut self) -> bool {
        self.board.try_move_x(1)
    }

    pub fn spin_left(&mut self) -> bool {
        self.board.try_spin(SpinDirection::Left)
    }

    pub fn spin_right(&mut self) -> bool {
        self.board.try_spin(SpinDirection::Right)
    }

    pub fn drop_softly(&mut self) -> DropResult {
        match self.board.drop_softly() {
            DroppingMinoStatus::InAir => {
                DropResult::SoftDropped
            }
            DroppingMinoStatus::OnGround => {
                self.put_and_spawn(); // TODO: handle error
                DropResult::Put
            }
        }
    }

    pub fn drop_hardly(&mut self) -> Option<usize> {
        self.board.drop_hardly()
            .filter(|_| self.put_and_spawn())
    }

    pub fn try_swap_hold(&mut self) {
        if !self.did_already_hold {
            self.did_already_hold = true;

            match self.hold_mino {
                Some(spawned) => {
                    let held = self.board.dropping;

                    self.hold_mino = Some(held);
                    self.board.spawn(spawned);
                }
                None => {
                    let spawned = self.bag.queue.pop_front().unwrap();
                    let held = self.board.dropping;

                    self.hold_mino = Some(held);
                    self.board.spawn(spawned);
                }
            }
        }
    }

    pub fn remove_lines(&mut self) {
        self.board.remove_lines();
    }

    fn put_and_spawn(&mut self) -> bool {
        self.board.determine_dropping_mino();

        self.did_already_hold = false;
        let mino = self.bag.pop();

        self.board.spawn(mino)
    }
}

pub struct MinoBag {
    queue: VecDeque<Tetrimino>
}

impl MinoBag {
    fn new() -> MinoBag {
        let mut queue = MinoBag::gen_shuffled_all_minos();
        let mut added = MinoBag::gen_shuffled_all_minos();
        queue.append(&mut added);

        MinoBag {
            queue: queue.into(),
        }
    }

    fn pop(&mut self) -> Tetrimino {
        let p = self.queue.pop_front().unwrap();

        if self.queue.len() < Tetrimino::all().len() {
            let added = MinoBag::gen_shuffled_all_minos();
            self.queue.append(&mut added.into());
        }

        p
    }

    pub fn peek(&self, amount: usize) -> Vec<Tetrimino> {
        if amount > Tetrimino::all().len() {
            panic!("the amount of minos must be equal to or lower than the amount of tetrimino types");
        }

        (0..amount)
            .map(|idx| self.queue.get(idx).unwrap().clone())
            .collect::<Vec<_>>()
    }


    fn gen_shuffled_all_minos() -> Vec<Tetrimino> {
        let mut rng = rand::thread_rng();

        let mut s = Tetrimino::all();
        s.shuffle(&mut rng);

        s
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum MinoBlock {
    AQUA,
    YELLOW,
    PURPLE,
    BLUE,
    ORANGE,
    GREEN,
    RED,
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

pub enum DropResult {
    SoftDropped,
    Put,
    Failed,
}

pub enum SpinDirection {
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Into<Point> for (isize, isize) {
    fn into(self) -> Point {
        Point { x: self.0, y: self.1 }
    }
}

impl Into<Point> for (f32, f32) {
    fn into(self) -> Point {
        Point { x: self.0 as isize, y: self.1 as isize }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_vec_returns_2d_vec() {
        let rect_vec: Vec<Vec<bool>> = rect_vec!(
        [0, 0, 0, 0],
        [1, 0, 1, 0],
        [0, 1, 0, 1],
        [1, 1, 1, 1],
    );

        assert_eq!(
            rect_vec,
            vec!(
                vec!(false, false, false, false),
                vec!(true, false, true, false),
                vec!(false, true, false, true),
                vec!(true, true, true, true),
            )
        );
    }

    #[test]
    fn gen_all_minos() {
        let mut minos = MinoBag::gen_shuffled_all_minos();

        assert_eq!(minos.len(), Tetrimino::all().len());

        minos.sort();
        minos.dedup();
        assert_eq!(minos.len(), Tetrimino::all().len());
    }

    #[test]
    fn peek_bag_minos() {
        let bag = MinoBag::new();
        let l = Tetrimino::all().len();

        assert_eq!(bag.peek(l).len(), l);
    }

    #[test]
    #[should_panic]
    fn peek_bag_minos_exceeded_limit() {
        let bag = MinoBag::new();
        let l = Tetrimino::all().len();

        bag.peek(l + 1);
    }
}